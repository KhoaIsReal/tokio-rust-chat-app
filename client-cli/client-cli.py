import socket
import threading

HOST = "127.0.0.1"
PORT = 8080

class Content:
    def __init__(self, username="", data=""):
        self.username = username
        self.data = data

def bytes_to_content(data: bytes) -> Content:
    parts = data.split(b"\x00", 1)
    username = parts[0].decode()

    if len(parts) > 1:
        msg_data = parts[1].decode()
    else:
        msg_data = ""

    return Content(username, msg_data)

def make_packet(username, data):
    return username.encode() + b"\x00" + data.encode()


def receive_loop(sock):
    """Receive messages from server forever."""
    while True:
        data = sock.recv(1024)
        if not data:
            print("Server closed connection.")
            break
        msg = bytes_to_content(data)
        print(f"\n{msg.username}: {msg.data}")


def send_loop(sock, username):
    """Allow user to type and send messages."""
    while True:
        msg = input()
        if msg.lower() == "/exit":
            break
        sock.sendall(make_packet(username, msg))


def main():
    username = input("Enter your username: ")

    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.connect((HOST, PORT))

    # Send initial hello
    s.sendall(make_packet(username, "joined the chat!"))

    # Start receiving thread
    threading.Thread(target=receive_loop, args=(s,), daemon=True).start()

    # Run sending loop (main thread)
    send_loop(s, username)

    s.close()


if __name__ == "__main__":
    main()
