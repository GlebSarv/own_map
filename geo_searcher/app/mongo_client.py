from motor.motor_asyncio import AsyncIOMotorClient, AsyncIOMotorCollection, AsyncIOMotorDatabase
from motor.core import AgnosticClient, AgnosticDatabase, AgnosticCollection
from config.mongodb_config import MongoConfig


class MongoClient():

    @classmethod
    async def run(cls, mc: MongoConfig, database: str=None, collection: str=None):
        
        self = MongoClient()
        self.url: str = mc.mongo_url
        self.database_name: str = database or mc.database
        self.collection_name: str = collection or mc.collection

        try:
            self.client: AgnosticClient = AsyncIOMotorClient(self.url)
            self.database: AgnosticDatabase = self.client[self.database_name]
            self.collection: AgnosticCollection = self.database[self.collection_name]
            return self 
        except Exception as e:
            print(e)


    async def do_insert(self, data: list[dict[str, str]]=None) -> bool: # TODO change annotate type
        
        result = await self.collection.insert_many([d for d in data])
        
        return len(result.inserted_ids) == len(data)
    
    async def do_find_one(self, k: str, v):
        document = await self.collection.find_one({k: v})
        return document

