from .config import CommonSettings
from pydantic_settings import SettingsConfigDict
from pydantic import computed_field

class KafkaConfig(CommonSettings):

    host: str
    port: int
    topic: str
    groupid: str

    model_config = SettingsConfigDict(
        env_prefix="kafka_"
    )

    @computed_field
    def bootstrap_server(self) -> str:

        return f"{self.host}:{self.port}"