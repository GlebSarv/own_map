from config.config import CommonSettings
from pydantic_settings import SettingsConfigDict
from pydantic import computed_field

class MongoConfig(CommonSettings):

    host: str
    port: int

    model_config = SettingsConfigDict(
        env_file="mongo_"
    )

    @computed_field
    def mongo_url(self) -> str:

        return f""