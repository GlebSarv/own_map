import json

from aiokafka import AIOKafkaConsumer
import asyncio

from config.kafka_config import KafkaConfig

class Consumer:

    def __init__(self, config: KafkaConfig) -> None:
        
        self.topic: str = config.topic
        self.broker: str = config.bootstrap_server
        self.group_id: str = config.groupid
        self.consumer: AIOKafkaConsumer = self.__get_consumer()
        
    def __get_consumer(self) -> AIOKafkaConsumer:        
        try:
            self.consumer: AIOKafkaConsumer = AIOKafkaConsumer(
                self.topic,
                bootstrap_servers=self.broker,
                group_id=self.group_id
            )
            return self.consumer
        except Exception as e:
            print(f"Error, {e}")
            
    async def start(self):
        try:
            await self.consumer.start()
        except Exception as e:
            print(f"Error, {e}",)

    async def stop(self):

        await self.consumer.stop()