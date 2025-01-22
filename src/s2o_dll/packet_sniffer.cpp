#include <winsock2.h>
#include <ws2tcpip.h>
#include <mstcpip.h>
#include <iostream>
#include <string>
#include <vector>
#include <mutex>
#include <chrono>
#include <atomic>
#include <thread>
#include <windows.h>

#pragma comment(lib, "ws2_32.lib")

static SOCKET raw_socket = INVALID_SOCKET;
static std::vector<std::vector<u_char>> captured_packets;
static std::mutex capture_mutex;
static std::atomic<bool> sniffing{false};
static std::thread capture_thread;

void capture_packets_aggressively() {
    char buffer[131072]; // Increased buffer size
    while (sniffing) {
        int bytes = recv(raw_socket, buffer, sizeof(buffer), 0);
        if (bytes > 0) {
            std::vector<u_char> packet(buffer, buffer + bytes);
            {
                std::lock_guard<std::mutex> guard(capture_mutex);
                captured_packets.push_back(std::move(packet));
            }
            std::cout << "[INFO] Packet captured. Size: " << bytes << " bytes" << std::endl;
        } else if (WSAGetLastError() != WSAEWOULDBLOCK) {
            std::cerr << "[ERROR] Packet capture error: " << WSAGetLastError() << std::endl;
        }
    }
}

extern "C" {
__declspec(dllexport) int start_sniffer() {
    std::cout << "[INFO] Starting aggressive sniffer." << std::endl;
    WSADATA wsaData;
    if (WSAStartup(MAKEWORD(2, 2), &wsaData) != 0) {
        std::cerr << "[ERROR] Failed to initialize Winsock." << std::endl;
        return -1;
    }

    raw_socket = socket(AF_INET, SOCK_RAW, IPPROTO_IP);
    if (raw_socket == INVALID_SOCKET) {
        std::cerr << "[ERROR] Failed to create raw socket. Error: " << WSAGetLastError() << std::endl;
        WSACleanup();
        return -2;
    }

    // Set socket to non-blocking
    u_long mode = 1;
    if (ioctlsocket(raw_socket, FIONBIO, &mode) == SOCKET_ERROR) {
        std::cerr << "[ERROR] Failed to set socket to non-blocking mode: " << WSAGetLastError() << std::endl;
        closesocket(raw_socket);
        WSACleanup();
        return -3;
    }

    // Set to promiscuous mode
    DWORD flag = 1;
    if (WSAIoctl(raw_socket, SIO_RCVALL, &flag, sizeof(flag), NULL, 0, NULL, NULL, NULL) == SOCKET_ERROR) {
        std::cerr << "[ERROR] Failed to set raw socket to promiscuous mode: " << WSAGetLastError() << std::endl;
        closesocket(raw_socket);
        WSACleanup();
        return -4;
    }

    sniffing = true;
    capture_thread = std::thread(capture_packets_aggressively);
    std::cout << "[INFO] Sniffer started in aggressive mode." << std::endl;
    return 0;
}

__declspec(dllexport) void stop_sniffer() {
    std::cout << "[INFO] Stopping sniffer." << std::endl;
    sniffing = false;
    if (capture_thread.joinable()) {
        capture_thread.join();
    }

    if (raw_socket != INVALID_SOCKET) {
        closesocket(raw_socket);
        raw_socket = INVALID_SOCKET;
    }
    WSACleanup();
    std::cout << "[INFO] Sniffer stopped." << std::endl;
}

__declspec(dllexport) const u_char* get_packet(int index) {
    std::lock_guard<std::mutex> guard(capture_mutex);
    if (index < 0 || index >= static_cast<int>(captured_packets.size())) {
        std::cerr << "[ERROR] Invalid packet index: " << index << std::endl;
        return NULL;
    }
    return captured_packets[index].data();
}

__declspec(dllexport) int get_packet_count() {
    std::lock_guard<std::mutex> guard(capture_mutex);
    return static_cast<int>(captured_packets.size());
}
}

// No GUI in this aggressive version to keep it lightweight