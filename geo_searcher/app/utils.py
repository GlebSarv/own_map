from geopy.adapters import AioHTTPAdapter
from geopy.geocoders import Nominatim
from geopy.exc import GeocoderTimedOut
from schema import GeoDataMessage

async def get_location(data: GeoDataMessage) -> dict:

    # TODO move this logic in ExifReader service
    if data.long == 0.0 and data.long == 0.0:
        return {"Error": "empty_coordinates"}
    
    async with Nominatim(user_agent="geo_searcher", adapter_factory=AioHTTPAdapter) as geolocator:
        try:
            location = await geolocator.reverse((data.lat, data.long), language="en")
            raw_location: dict = location.raw['address']
            
            if raw_location.get("ISO3166-2-lvl4") is not None:
                raw_location['iso'] = raw_location.get("ISO3166-2-lvl4")
        except GeocoderTimedOut as gto:
            await get_location(data=data)
        return raw_location