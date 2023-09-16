from pydantic import BaseModel
from typing import Optional

class GeoData(BaseModel):

    region: Optional[str] = None
    state: Optional[str] = None
    country: str
    country_code: str
    iso: str
    city: Optional[str] = None


class GeoDataMessage(BaseModel):

    lat: float
    long: float