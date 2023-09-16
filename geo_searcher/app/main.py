from config.kafka_config import KafkaConfig
from config.mongodb_config import MongoConfig
from consumer import Consumer
from mongo_client import MongoClient
from app_facade import App
import asyncio 

async def async_main():

    kc: KafkaConfig = KafkaConfig()
    mc: MongoConfig = MongoConfig()

    consumer: Consumer = Consumer(config=kc)
    mongo_client: MongoClient = await MongoClient.run(mc=mc)

    app: App = App(consumer=consumer, mongo_client=mongo_client)
    await app.save_location()

if __name__ == "__main__":

    asyncio.run(async_main())