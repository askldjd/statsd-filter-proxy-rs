from threading import Thread
import socket
import sys
from time import sleep

TEST_MSG_COUNT = 1000

class Sender(Thread):
    def run(self):
        sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        server_address = ('127.0.0.1', 8125)
        try:
            num_send = 0

            while True:
                message = f'{num_send} - This is the message.  It will be repeated.'
                sock.sendto(str.encode(message), server_address)
                num_send += 1

                if num_send % 100 == 0:
                    print(f"sent {num_send} messages")

                sleep(0.01)

                if num_send > TEST_MSG_COUNT:
                    break

        finally:
            print('closing socket')
            sock.close()


class Receiver(Thread):
    def run(self):
        UDP_IP = "127.0.0.1"
        UDP_PORT = 8126

        sock = socket.socket(socket.AF_INET,  # Internet
                             socket.SOCK_DGRAM)  # UDP
        sock.bind((UDP_IP, UDP_PORT))

        received_data = set()

        while True:
            data, addr = sock.recvfrom(8192)
            received_data.add(data)

            if len(received_data) % 100 == 0:
                print(f"received {len(received_data)} messages")

            if len(received_data) >= TEST_MSG_COUNT:
                print(f"received {TEST_MSG_COUNT} message, exiting")
                break


def main():
    receiver = Receiver()
    sender = Sender()
    
    receiver.start()
    print("started receiver thread, waiting 5s")
    sleep(5)
    sender.start()
    print("started sender thread, testing in progress")

    receiver.join()
    sender.join()
    print("test completed")


if __name__ == '__main__':
    main()
