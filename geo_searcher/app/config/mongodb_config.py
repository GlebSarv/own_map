from .config import CommonSettings
from pydantic_settings import SettingsConfigDict
from pydantic import computed_field


class MongoConfig(CommonSettings):

    host: str
    port: int
    database: str
    collection: str

    model_config = SettingsConfigDict(
        env_prefix="mongo_"
    )

    @computed_field
    def mongo_url(self) -> str:

        return f"mongodb://{self.host}:{self.port}"