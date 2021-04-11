from threading import Thread
import socket
import sys
from time import time, sleep
import numpy as np

TEST_MSG_COUNT = 1000


class Sender(Thread):
    def run(self):
        sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        server_address = ('127.0.0.1', 8125)
        try:
            num_send = 0

            while True:
                message = f'{time()}'
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

        latencies = []

        while True:
            data, addr = sock.recvfrom(8192)
            now = time()
            then = float(data)
            took = now - then

            latencies.append(took)

            if len(latencies) % 100 == 0:
                print(f"received {len(latencies)} messages")

            if len(latencies) >= TEST_MSG_COUNT:
                print(f"received {TEST_MSG_COUNT} message, exiting")
                break

        np_latencies = np.array(latencies)
        print(f"median = {np.percentile(np_latencies, 50)*1000000} us")
        print(f"p95 = {np.percentile(np_latencies, 95)*1000000} us")


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
