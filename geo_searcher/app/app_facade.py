from consumer import Consumer
from mongo_client import MongoClient
from utils import get_location
from schema import GeoData, GeoDataMessage
import json


class App:

    def __init__(self, consumer: Consumer, mongo_client: MongoClient) -> None:
        
        self.consumer: Consumer = consumer
        self.mongo_client: MongoClient = mongo_client

    async def save_location(self):
        try:

            await self.consumer.start()
            async for msg in self.consumer.consumer: # TODO Thinking about insert batch ??
                data = json.loads(msg.value.decode('utf8'))
                gdm: GeoDataMessage = GeoDataMessage(**data)
                location = await get_location(data=gdm) 
                if location.get("Error") is not None: # TODO move validation to ExifReader
                    continue
                
                gd = GeoData(**location)
                await self.mongo_client.do_insert(data=[gd.__dict__])
            
        except Exception as e:
            print(f"Error {e}")
            
        finally:
            await self.consumer.stop()