import digital_twin_get_provider_pb2 as dt

def main():
    msg = dt.GetRequest()
    msg.entity_id = "1111"
    print(msg)
    
    
if __name__ == "__main__":
    exit(main())