import pytest
import pytest_asyncio
import asyncio
from typing import Generator

from ..app.config.kafka_config import KafkaConfig
from ..app.config.mongodb_config import MongoConfig
from ..app.mongo_client import MongoClient

@pytest.fixture(scope='session')
def event_loop() -> Generator:

    loop = asyncio.get_event_loop_policy().new_event_loop()
    yield loop
    loop.close()

@pytest.fixture(scope='session')
def get_configuration() -> dict:

    config: dict[str, KafkaConfig | MongoConfig] = {
        'kafka': KafkaConfig(),
        'mongo': MongoConfig()
    }

    return config

@pytest_asyncio.fixture(scope='session')
async def create_mongo_db(get_configuration):
    
    mongo_config: MongoConfig = get_configuration.get('mongo')

    mc: MongoClient  = await MongoClient.run(mc=mongo_config, database="test_database", collection="test_collection")
    
    yield mc
