#include <winsock2.h>
#include <ws2tcpip.h>
#include <mstcpip.h>
#include <stdio.h>

// Link with ws2_32.lib
#pragma comment(lib, "Ws2_32.lib")

#ifndef SIO_RCVALL
#define SIO_RCVALL _WSAIOW(IOC_VENDOR,1)
#endif

extern "C" {
    __declspec(dllexport) int init_sniffer() {
        // Initialize Winsock
        WSADATA wsaData;
        int result = WSAStartup(MAKEWORD(2, 2), &wsaData);
        if (result != 0) {
            fprintf(stderr, "Failed to initialize Winsock: %d\n", result);
            return 1;
        }
        return 0;
    }

    __declspec(dllexport) void run_sniffer(void (*callback)(const unsigned char*, int)) {
        // Create a socket for packet capture
        SOCKET captureSocket = socket(AF_INET, SOCK_RAW, IPPROTO_IP);
        if (captureSocket == INVALID_SOCKET) {
            fprintf(stderr, "Socket creation failed: %d\n", WSAGetLastError());
            goto cleanup;
        }

        // Enable receiving of the IP header
        BOOL optval = TRUE;
        if (setsockopt(captureSocket, IPPROTO_IP, IP_HDRINCL, (char*)&optval, sizeof(optval)) == SOCKET_ERROR) {
            fprintf(stderr, "Failed to set socket option: %d\n", WSAGetLastError());
            goto cleanup_socket;
        }

        // Bind the socket to any interface
        SOCKADDR_IN sockAddr;
        sockAddr.sin_family = AF_INET;
        sockAddr.sin_port = 0;
        sockAddr.sin_addr.s_addr = htonl(INADDR_ANY);

        if (bind(captureSocket, (SOCKADDR*)&sockAddr, sizeof(sockAddr)) == SOCKET_ERROR) {
            fprintf(stderr, "Binding failed: %d\n", WSAGetLastError());
            goto cleanup_socket;
        }

        // Enable promiscuous mode
        DWORD dwValue = 1;
        if (WSAIoctl(captureSocket, SIO_RCVALL, &dwValue, sizeof(dwValue), NULL, 0, NULL, NULL, NULL) == SOCKET_ERROR) {
            fprintf(stderr, "Failed to set promiscuous mode: %d\n", WSAGetLastError());
            goto cleanup_socket;
        }

        // Packet capture loop
        const int bufferSize = 65536;
        char buffer[bufferSize];
        int bytesRead;
        while (true) {
            bytesRead = recv(captureSocket, buffer, bufferSize, 0);
            if (bytesRead == SOCKET_ERROR) {
                fprintf(stderr, "Receive operation failed: %d\n", WSAGetLastError());
                break;
            }
            callback((const unsigned char*)buffer, bytesRead);
        }

    cleanup_socket:
        closesocket(captureSocket);
    cleanup:
        WSACleanup();
    }
}