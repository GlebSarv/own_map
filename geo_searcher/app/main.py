from config.kafka_config import KafkaConfig
from config.mongodb_config import MongoConfig


if __name__ == "__main__":

  #  kc = KafkaConfig()
    mc = MongoConfig()

    print(mc.__dict__)