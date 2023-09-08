from pydantic_settings import SettingsConfigDict, BaseSettings
from pathlib import Path
import os

BASE_DIR: Path = Path(__file__).absolute().parent.parent.parent
file_name: str = ".env"


class CommonSettings(BaseSettings):
   
    if os.path.exists(f"{BASE_DIR/file_name}"):
        print(f"{BASE_DIR/file_name}")
        model_config = SettingsConfigDict (
            env_file=f"{BASE_DIR}/{file_name}",
            env_file_encoding="utf-8"
        )
    else:
        model_config = SettingsConfigDict(
            env_file=None,
            env_file_encoding="utf-8"
        )

