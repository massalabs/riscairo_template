from starknet_py.net.full_node_client import FullNodeClient
from starknet_py.contract import Contract
import asyncio
import aiohttp
import hashlib

async def main():
    node_url = "https://rpc.nethermind.io/sepolia-juno"
    api_key = "YOUR_API_KEY"
    contract_addr = "0x06e5eab47f50d1817a5e56d52b54a93f04bb173abd835283419bebe841d54d6c"
    headers = {
        "x-apikey": api_key
    }

    async with aiohttp.ClientSession(headers=headers) as session:
        cli = FullNodeClient(node_url=node_url, session=session)

        contract = await Contract.from_address(
            address=contract_addr,
            provider=cli,
        )
        
        # Call the contract functions

        print("Adding two numbers using rust to demonstrate basic arithmetics:")
        a = 12
        b = 56
        res = await contract.functions["add"].call(a, b)
        print(f" {a} + {b} = {res[0]}")

        print("Prepending text using rust to demonstrate guest dynamic allocation:")
        base_data = "world!"

        data = bytes((await contract.functions["prepend_hello"].call(base_data.encode('utf-8')))[0]).decode("utf-8") 
        print("  'hello ' + 'world!' = '" + data + "'")
        
        print("Computing the blake2s256('" + data + "') hash:")
        print("  Computing using the blake2 rust crate from Cairo contract:")
        result = bytes((await contract.functions["compute_hash"].call(data.encode('utf-8')))[0]).hex()
        print("    Result:", result)
        print("  Computing locally:")
        result = hashlib.blake2s(data.encode('utf-8'), digest_size=32).hexdigest()
        print("    Result:", result)

        print("Making the rust guest panic to demonstrate guest error handling:")
        a = 150
        b = 200  # sum overflows u8 and causes a guest panic
        print(f"  Trying to add {a} and {b} which overflows the expected u8 result...")
        try:
            await contract.functions["add"].call(a, b)
        except Exception as e:
            print("    Error:", e)

if __name__ == "__main__":
    asyncio.run(main())

