import socket
import cbor
client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

client.connect(('0.0.0.0', 6767))
payload = {"ts": 186401, "app_name": "09", "app_layer": "we", "value": 4.4332, "operation1": "select", 
"metric_name": "2323", 
"labels": {"app_name": "3443","HOST": "host1", "port": "0987"}}
# data = (payload * 3).encode() 
# print(cbor.dumps(payload)+b'\n')
data = cbor.dumps(payload) + '\n'.encode()
while True:
    client.send(data)
    client.recv(1024)
