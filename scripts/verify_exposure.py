import socket
import json
import time

def send_request(method, path, body=None):
    host = "127.0.0.1"
    port = 8081
    
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((host, port))
        
        request = f"{method} {path} HTTP/1.1\r\n"
        request += f"Host: {host}\r\n"
        if body:
            request += f"Content-Length: {len(body)}\r\n"
            request += "Content-Type: application/json\r\n"
        request += "\r\n"
        if body:
            request += body
            
        s.sendall(request.encode())
        
        response = s.recv(4096).decode()
        return response

def test_schema():
    print("Testing GET /schema...")
    response = send_request("GET", "/schema")
    if "HTTP/1.1 200 OK" in response:
        print("Success: Schema retrieved.")
        # Try to parse the body
        try:
            body = response.split("\r\n\r\n")[1]
            schema = json.loads(body)
            print(f"Schema contains {len(schema.get('components', []))} components.")
        except Exception as e:
            print(f"Error parsing schema JSON: {e}")
    else:
        print(f"Failed: {response}")

def test_command():
    print("\nTesting POST /command (Navigate to SettingsAI)...")
    command = {
        "SetTab": "SettingsAI"
    }
    response = send_request("POST", "/command", json.dumps(command))
    if "HTTP/1.1 200 OK" in response:
        print("Success: Command accepted.")
    else:
        print(f"Failed: {response}")

def test_instructions():
    print("\nTesting GET /instructions...")
    response = send_request("GET", "/instructions")
    if "HTTP/1.1 200 OK" in response:
        print("Success: Instructions retrieved.")
        if "PeakUI Neural Exposure Protocol" in response:
            print("Instruction content verified.")
    else:
        print(f"Failed: {response}")

def test_view():
    print("\nTesting GET /view...")
    response = send_request("GET", "/view")
    if "HTTP/1.1 200 OK" in response:
        print("Success: Live view retrieved.")
        try:
            body = response.split("\r\n\r\n")[1]
            view = json.loads(body)
            print(f"Current UI root: {view.get('node_type')}")
            print(f"Current Page Label: {view.get('label')}")
        except Exception as e:
            print(f"Error parsing view JSON: {e}")
    else:
        print(f"Failed: {response}")

if __name__ == "__main__":
    print("PeakUI API Verification Script")
    print("Make sure the application is running and 'Neural Exposure' is enabled in Settings -> AI.")
    try:
        test_schema()
        test_instructions()
        test_view()
        test_command()
    except ConnectionRefusedError:
        print("Error: Could not connect to the API server. Is the app running and Exposure enabled?")
    except Exception as e:
        print(f"An error occurred: {e}")
