import websocket
import threading
import json

def on_message(ws, message):
    print("Received:", message)

def on_error(ws, error):
    print("Error:", error)

def on_close(ws, close_status_code, close_msg):
    print("WebSocket closed")

def on_open(ws):
    print("WebSocket connection opened")
    msg = {
        "type": "message",
        "content": "Hello, room!"
    }
    ws.send(json.dumps(msg))

if __name__ == "__main__":
    room_id = input("Enter room ID: ").strip()
    jwt_token = input("Enter JWT token: ").strip()
    ws_url = f"ws://localhost:8080/api/v1/ws/?room_id={room_id}"

    headers = {
        "Authorization": f"Bearer {jwt_token}"
    }
    ws = websocket.WebSocketApp(
        ws_url,
        header=[f"{k}: {v}" for k, v in headers.items()],
        on_open=on_open,
        on_message=on_message,
        on_error=on_error,
        on_close=on_close
    )

    wst = threading.Thread(target=ws.run_forever)
    wst.daemon = True
    wst.start()

    try:
        while True:
            msg = input("Type message (or 'exit' to quit): ")
            if msg.lower() == "exit":
                break
            ws.send(json.dumps({"type": "message", "content": msg}))
    except KeyboardInterrupt:
        pass
    ws.close()