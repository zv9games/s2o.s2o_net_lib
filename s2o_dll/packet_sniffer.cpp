#include <winsock2.h>
#include <ws2tcpip.h>
#include <iostream>

#pragma comment(lib, "Ws2_32.lib")

#ifndef SIO_RCVALL
#define SIO_RCVALL _WSAIOW(IOC_VENDOR,1)
#endif

#ifndef RCVALL_ON
#define RCVALL_ON  1
#endif

#ifndef RCVALL_OFF
#define RCVALL_OFF 0
#endif

extern "C" {
    __declspec(dllexport) void start_sniffer(const char* ip_address) {
        WSADATA wsaData;
        int result;

        // Initialize Winsock
        result = WSAStartup(MAKEWORD(2, 2), &wsaData);
        if (result != 0) {
            std::cerr << "WSAStartup failed: " << result << std::endl;
            return;
        }

        // Create a raw socket
        SOCKET sniffer = socket(AF_INET, SOCK_RAW, IPPROTO_IP);
        if (sniffer == INVALID_SOCKET) {
            std::cerr << "socket failed: " << WSAGetLastError() << std::endl;
            WSACleanup();
            return;
        }

        // Bind the socket
        sockaddr_in dest;
        dest.sin_family = AF_INET;
        dest.sin_port = htons(0); // Any port
        dest.sin_addr.s_addr = inet_addr(ip_address); // Passed IP address

        result = bind(sniffer, (struct sockaddr *)&dest, sizeof(dest));
        if (result == SOCKET_ERROR) {
            std::cerr << "bind failed: " << WSAGetLastError() << std::endl;
            closesocket(sniffer);
            WSACleanup();
            return;
        }

        // Set the socket to promiscuous mode
        DWORD dwValue = RCVALL_ON;
        result = ioctlsocket(sniffer, SIO_RCVALL, &dwValue);
        if (result == SOCKET_ERROR) {
            std::cerr << "ioctlsocket failed: " << WSAGetLastError() << std::endl;
            closesocket(sniffer);
            WSACleanup();
            return;
        }

        // Start capturing packets
        char buffer[65536];
        while (true) {
            int bytesReceived = recvfrom(sniffer, buffer, sizeof(buffer), 0, nullptr, nullptr);
            if (bytesReceived > 0) {
                std::cout << "Captured packet: " << bytesReceived << " bytes" << std::endl;
            } else if (bytesReceived == 0) {
                std::cout << "Connection closed" << std::endl;
                break;
            } else {
                std::cerr << "recvfrom failed: " << WSAGetLastError() << std::endl;
                break;
            }
        }

        // Cleanup
        closesocket(sniffer);
        WSACleanup();
    }
}
