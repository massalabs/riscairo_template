from starknet_py.net.full_node_client import FullNodeClient
from starknet_py.contract import Contract
import asyncio
import aiohttp
import hashlib

async def main():
    node_url = "https://rpc.nethermind.io/sepolia-juno"
    api_key = "YOUR_API_KEY"
    contract_addr = "0x0089a8b091280ac9e82e3954a7bd30cba6fcb1e2f63370d40567b3227073aaaa"
    headers = {
        "x-apikey": api_key
    }

    async with aiohttp.ClientSession(headers=headers) as session:
        cli = FullNodeClient(node_url=node_url, session=session)

        contract = await Contract.from_address(
            address=contract_addr,
            provider=cli,
        )

        data = "hello world!"

        # Call the contract function asynchronously
        print("Computing blake2s_256('" + data + "') locally")
        result = hashlib.blake2s(data.encode('utf-8'), digest_size=32).hexdigest()
        print("Result:", result)
        print("Computing using blake2 rust crate from cairo contract", contract_addr)
        result = bytes((await contract.functions["compute_hash"].call(data.encode('utf-8')))[0]).hex()
        print("Result:", result)


if __name__ == "__main__":
    asyncio.run(main())

