import aiohttp
import asyncio
import uvloop


async def fetch(session, url):
    async with session.get(url) as response:
        if response.status != 200:
            print(f"failed with status code {response.status}")
        return await response.text()


async def fetch_all(urls):
    async with aiohttp.ClientSession() as session:
        texts = await asyncio.gather(*[fetch(session, url) for url in urls])
        return texts


uvloop.install()  # speeds up async

urls_to_fetch = ["http://0.0.0.0:5000" for x in range(10000)]

response = asyncio.run(fetch_all(urls_to_fetch))
print(len(response))
