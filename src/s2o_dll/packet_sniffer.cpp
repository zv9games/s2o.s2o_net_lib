#include <winsock2.h>
#include <ws2tcpip.h>
#include <mstcpip.h>
#include <iphlpapi.h>
#include <stdio.h>

// Link with ws2_32.lib and Iphlpapi.lib
#pragma comment(lib, "Ws2_32.lib")
#pragma comment(lib, "Iphlpapi.lib")

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
        printf("Winsock initialized successfully.\n");
        return 0;
    }

    __declspec(dllexport) void run_sniffer(void (*callback)(const unsigned char*, int)) {
        // Create a socket for packet capture
        SOCKET captureSocket = socket(AF_INET, SOCK_RAW, IPPROTO_IP);
        if (captureSocket == INVALID_SOCKET) {
            fprintf(stderr, "Socket creation failed: %d\n", WSAGetLastError());
            return;
        }
        printf("Socket created successfully.\n");

        // Enable receiving of the IP header
        BOOL optval = TRUE;
        if (setsockopt(captureSocket, IPPROTO_IP, IP_HDRINCL, (char*)&optval, sizeof(optval)) == SOCKET_ERROR) {
            fprintf(stderr, "Failed to set socket option: %d\n", WSAGetLastError());
            closesocket(captureSocket);
            return;
        }
        printf("Socket option set successfully.\n");

        // Retrieve and print adapters
        PIP_ADAPTER_INFO adapterInfo = (IP_ADAPTER_INFO*)malloc(sizeof(IP_ADAPTER_INFO));
        ULONG adapterInfoSize = sizeof(IP_ADAPTER_INFO);
        if (GetAdaptersInfo(adapterInfo, &adapterInfoSize) == ERROR_BUFFER_OVERFLOW) {
            adapterInfo = (IP_ADAPTER_INFO*)malloc(adapterInfoSize);
        }
        if (GetAdaptersInfo(adapterInfo, &adapterInfoSize) == NO_ERROR) {
            PIP_ADAPTER_INFO adapter = adapterInfo;
            while (adapter) {
                printf("Adapter: %s\n", adapter->Description);
                adapter = adapter->Next;
            }
        }

        // Bind the socket to a specific adapter's IP address
        IP_ADAPTER_INFO* adapter = adapterInfo;
        while (adapter) {
            if (adapter->Type == MIB_IF_TYPE_ETHERNET && adapter->IpAddressList.IpAddress.String[0] != '\0') {
                printf("Binding to adapter: %s with IP: %s\n", adapter->Description, adapter->IpAddressList.IpAddress.String);
                break;
            }
            adapter = adapter->Next;
        }

        if (!adapter) {
            fprintf(stderr, "No suitable adapter found.\n");
            closesocket(captureSocket);
            return;
        }

        SOCKADDR_IN sockAddr;
        sockAddr.sin_family = AF_INET;
        sockAddr.sin_port = 0;
        sockAddr.sin_addr.s_addr = inet_addr(adapter->IpAddressList.IpAddress.String);

        if (bind(captureSocket, (SOCKADDR*)&sockAddr, sizeof(sockAddr)) == SOCKET_ERROR) {
            fprintf(stderr, "Binding failed: %d\n", WSAGetLastError());
            closesocket(captureSocket);
            return;
        }
        printf("Socket bound successfully to %s.\n", adapter->IpAddressList.IpAddress.String);

        free(adapterInfo);

        // Enable promiscuous mode
        DWORD dwValue = 1;
        DWORD bytesReturned = 0;
        printf("Attempting to set promiscuous mode on socket %lld\n", (long long)captureSocket);
        if (WSAIoctl(captureSocket, SIO_RCVALL, &dwValue, sizeof(dwValue), NULL, 0, &bytesReturned, NULL, NULL) == SOCKET_ERROR) {
            fprintf(stderr, "Failed to set promiscuous mode: %d\n", WSAGetLastError());
            closesocket(captureSocket);
            return;
        }
        printf("Promiscuous mode enabled successfully.\n");

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
            printf("Packet received: %d bytes\n", bytesRead);
            callback((const unsigned char*)buffer, bytesRead);
        }

        closesocket(captureSocket);
        WSACleanup();
    }

    __declspec(dllexport) int stop_sniffer() {
        // Implement stop sniffer logic here
        printf("Sniffer stopped successfully.\n");
        return 0; // Placeholder return value
    }

    __declspec(dllexport) int capture_packet_data() {
        // Create a socket for packet capture
        SOCKET captureSocket = socket(AF_INET, SOCK_RAW, IPPROTO_IP);
        if (captureSocket == INVALID_SOCKET) {
            fprintf(stderr, "Socket creation failed: %d\n", WSAGetLastError());
            return 1;
        }
        printf("Socket created successfully.\n");

        // Enable receiving of the IP header
        BOOL optval = TRUE;
        if (setsockopt(captureSocket, IPPROTO_IP, IP_HDRINCL, (char*)&optval, sizeof(optval)) == SOCKET_ERROR) {
            fprintf(stderr, "Failed to set socket option: %d\n", WSAGetLastError());
            closesocket(captureSocket);
            return 1;
        }
        printf("Socket option set successfully.\n");

        // Retrieve and print adapters
        PIP_ADAPTER_INFO adapterInfo = (IP_ADAPTER_INFO*)malloc(sizeof(IP_ADAPTER_INFO));
        ULONG adapterInfoSize = sizeof(IP_ADAPTER_INFO);
        if (GetAdaptersInfo(adapterInfo, &adapterInfoSize) == ERROR_BUFFER_OVERFLOW) {
            adapterInfo = (IP_ADAPTER_INFO*)malloc(adapterInfoSize);
        }
        if (GetAdaptersInfo(adapterInfo, &adapterInfoSize) == NO_ERROR) {
            PIP_ADAPTER_INFO adapter = adapterInfo;
            while (adapter) {
                printf("Adapter: %s\n", adapter->Description);
                adapter = adapter->Next;
            }
        }

        // Bind the socket to a specific adapter's IP address
        IP_ADAPTER_INFO* adapter = adapterInfo;
        while (adapter) {
            if (adapter->Type == MIB_IF_TYPE_ETHERNET && adapter->IpAddressList.IpAddress.String[0] != '\0') {
                printf("Binding to adapter: %s with IP: %s\n", adapter->Description, adapter->IpAddressList.IpAddress.String);
                break;
            }
            adapter = adapter->Next;
        }

        if (!adapter) {
            fprintf(stderr, "No suitable adapter found.\n");
            closesocket(captureSocket);
            return 1;
        }

        SOCKADDR_IN sockAddr;
        sockAddr.sin_family = AF_INET;
        sockAddr.sin_port = 0;
        sockAddr.sin_addr.s_addr = inet_addr(adapter->IpAddressList.IpAddress.String);

        if (bind(captureSocket, (SOCKADDR*)&sockAddr, sizeof(sockAddr)) == SOCKET_ERROR) {
            fprintf(stderr, "Binding failed: %d\n", WSAGetLastError());
            closesocket(captureSocket);
            return 1;
        }
        printf("Socket bound successfully to %s.\n", adapter->IpAddressList.IpAddress.String);

        free(adapterInfo);

        // Enable promiscuous mode
        DWORD dwValue = 1;
        DWORD bytesReturned = 0;
        printf("Attempting to set promiscuous mode on socket %lld\n", (long long)captureSocket);
        if (WSAIoctl(captureSocket, SIO_RCVALL, &dwValue, sizeof(dwValue), NULL, 0, &bytesReturned, NULL, NULL) == SOCKET_ERROR) {
            fprintf(stderr, "Failed to set promiscuous mode: %d\n", WSAGetLastError());
            closesocket(captureSocket);
            return 1;
        }
        printf("Promiscuous mode enabled successfully.\n");

        // Packet capture loop (you can adjust the termination condition as needed)
        const int bufferSize = 65536;
        char buffer[bufferSize];
        int bytesRead;
        while (true) {
            bytesRead = recv(captureSocket, buffer, bufferSize, 0);
            if (bytesRead == SOCKET_ERROR) {
                fprintf(stderr, "Receive operation failed: %d\n", WSAGetLastError());
                break;
            }
            printf("Packet received: %d bytes\n", bytesRead);
        }

        closesocket(captureSocket);
        WSACleanup();

        return 0;
    }

    __declspec(dllexport) int get_captured_packet_count() {
        // Implement packet count retrieval logic here
        return 0; // Placeholder return value
    }

    __declspec(dllexport) const unsigned char* get_captured_packet(int index) {
        // Implement captured packet retrieval logic here
        return nullptr; // Placeholder return value
    }
}
