import pytest
from unittest.mock import Mock, patch
from ..app.utils import get_location
from ..app.schema import GeoData, GeoDataMessage
from ..app.mongo_client import MongoClient

geo_data_list = [
    (55.7558, 37.6176, 'Ru', {'historic': 'Kremlin and Red Square, Moscow', 'road': 'Красное крыльцо', 'quarter': '18', 'suburb': 'Tverskoy District', 'city': 'Moscow', 'ISO3166-2-lvl15': 'RU-MOW', 'state': 'Moscow', 'iso': 'RU-MOW', 'region': 'Central Federal District', 'postcode': '103073', 'country': 'Russia', 'country_code': 'ru'}, {'region': 'Central Federal District', 'state': 'Moscow', 'country': 'Russia', 'country_code': 'ru', 'iso': 'RU-MOW', 'city': 'Moscow'}),
    
    (59.9343, 30.3351, 'Ru', {'amenity': 'Театр марионеток им. Е. С. Деммени', 'house_number': '52', 'road': 'Nevski avenue', 'city_district': 'Palace District', 'city': 'Saint Petersburg', 'ISO3166-2-lvl15': 'RU-SPE', 'state': 'Saint Petersburg', 'iso': 'RU-SPE', 'region': 'Northwestern Federal District', 'postcode': '191168', 'country': 'Russia', 'country_code': 'ru'}, {'region': 'Northwestern Federal District', 'state': 'Saint Petersburg', 'country': 'Russia', 'country_code': 'ru', 'iso': 'RU-SPE', 'city': 'Saint Petersburg'}),
    
    (56.8385, 60.6055, 'Ru', {'road': 'Lenin Avenue', 'city_district': 'Leninsky District', 'city': 'Yekaterinburg', 'county': 'Yekaterinburg Municipality', 'state': 'Sverdlovsk Oblast', 'iso': 'RU-SVE', 'region': 'Ural Federal District', 'postcode': '620151', 'country': 'Russia', 'country_code': 'ru'}, {'region': 'Ural Federal District', 'state': 'Sverdlovsk Oblast', 'country': 'Russia', 'country_code': 'ru', 'iso': 'RU-SVE', 'city': 'Yekaterinburg'}),
    
    (55.7558, 49.1917, 'Ru', {'amenity': 'Детский сад № 342 «Одуванчик»', 'road': 'Bratyev Kasimovykh Street', 'suburb': 'Горки-1', 'city_district': 'Приволжский район', 'city': 'Kazan', 'state': 'Tatarstan', 'iso': 'RU-TA', 'region': 'Volga Federal District', 'postcode': '420101', 'country': 'Russia', 'country_code': 'ru'}, {'region': 'Volga Federal District', 'state': 'Tatarstan', 'country': 'Russia', 'country_code': 'ru', 'iso': 'RU-TA', 'city': 'Kazan'}),
    
    (51.1657, 71.4344, 'Kz', {'house_number': '11', 'road': 'улица Иманбаевой', 'city_district': 'Bayqoñır district', 'city': 'Astana', 'iso': 'KZ-71', 'postcode': '010000', 'country': 'Kazakhstan', 'country_code': 'kz'}, {'region': None, 'state': None, 'country': 'Kazakhstan', 'country_code': 'kz', 'iso': 'KZ-71', 'city': 'Astana'}),
    
    (45.0355, 38.9753, 'Ru', {'amenity': 'Цвето-музыкальный фонтан', 'road': 'Budionnogo Street', 'city_district': 'Центральный округ', 'city': 'Krasnodar', 'county': 'Krasnodar Municipality', 'state': 'Krasnodar Krai', 'iso': 'RU-KDA', 'region': 'Southern Federal District', 'postcode': '350000', 'country': 'Russia', 'country_code': 'ru'}, {'region': 'Southern Federal District', 'state': 'Krasnodar Krai', 'country': 'Russia', 'country_code': 'ru', 'iso': 'RU-KDA', 'city': 'Krasnodar'}),
    
    (53.9045, 27.5615, 'By', {'building': 'Бізнес-цэнтр «Colliers International»', 'house_number': '36', 'road': 'Інтэрнацыянальная вуліца', 'neighbourhood': 'Valoki Polackija', 'city_district': 'Tsentralny District', 'city': 'Minsk', 'iso': 'BY-HM', 'postcode': '220030', 'country': 'Belarus', 'country_code': 'by'}, {'region': None, 'state': None, 'country': 'Belarus', 'country_code': 'by', 'iso': 'BY-HM', 'city': 'Minsk'}),
    
    (48.8566, 2.3522, 'Fr', {'amenity': 'Hôtel de Ville', 'road': "Place de l'Hôtel de Ville", 'city_block': 'Quartier Saint-Merri', 'suburb': '4th Arrondissement', 'city_district': 'Paris', 'city': 'Paris', 'ISO3166-2-lvl6': 'FR-75', 'state': 'Ile-de-France', 'iso': 'FR-IDF', 'region': 'Metropolitan France', 'postcode': '75004', 'country': 'France', 'country_code': 'fr'}, {'region': 'Metropolitan France', 'state': 'Ile-de-France', 'country': 'France', 'country_code': 'fr', 'iso': 'FR-IDF', 'city': 'Paris'}),
    
    (40.7128, -74.0060, 'Us', {'amenity': 'New York City Hall', 'house_number': '260', 'road': 'Broadway', 'quarter': 'Lower Manhattan', 'neighbourhood': 'Manhattan Community Board 1', 'suburb': 'Manhattan', 'county': 'New York County', 'city': 'New York', 'state': 'New York', 'iso': 'US-NY', 'postcode': '10000', 'country': 'United States', 'country_code': 'us'}, {'region': None, 'state': 'New York', 'country': 'United States', 'country_code': 'us', 'iso': 'US-NY', 'city': 'New York'}),
    
    (37.7749, -122.4194, 'Us', {'road': 'South Van Ness Avenue', 'neighbourhood': 'Hayes Valley', 'city': 'San Francisco', 'state': 'California', 'iso': 'US-CA', 'postcode': '94103', 'country': 'United States', 'country_code': 'us'}, {'region': None, 'state': 'California', 'country': 'United States', 'country_code': 'us', 'iso': 'US-CA', 'city': 'San Francisco'}),
]


@pytest.mark.parametrize('coordinates', geo_data_list)
@pytest.mark.unit
def test_create_geo_data_message(coordinates: tuple[int, int, str, dict[str, str], dict[str, str]]):

    lat, long = coordinates[0:2]

    gdm = GeoDataMessage(lat=lat, long=long)
    assert gdm.lat == lat and gdm.long == long


@pytest.mark.asyncio
@pytest.mark.parametrize('coordinates', geo_data_list)
@pytest.mark.unit
async def test_get_location(coordinates: tuple[int, int, str, dict[str, str], dict[str, str]]):
    
    latitude, longitude, expectation_code, raw_geo_data = coordinates[0:4]
    gdm: GeoDataMessage = GeoDataMessage(lat=latitude, long=longitude)
    mock_geolocator = Mock()
    mock_geolocator.reverse= {
        'address': 'Your fake address data'
    }

    with patch('geopy.geocoders.Nominatim', return_value=mock_geolocator):
        
        location = await get_location(gdm)
        assert location.get('country') == raw_geo_data.get('country')
        assert location.get('city') == raw_geo_data.get('city')
        assert location.get('state') == raw_geo_data.get('state')
        assert location.get('region') == raw_geo_data.get('region')
        assert location.get('iso') == raw_geo_data.get('iso')
        
        country_code: str = location['country_code']
        assert country_code.lower() == expectation_code.lower()


@pytest.mark.parametrize('geo_data', geo_data_list)
@pytest.mark.unit
def test_create_geo_data(geo_data: tuple[int, int, str, dict[str, str], dict[str, str]]):
    raw_geo_data, expectation = geo_data[3:5]
    gd: GeoData = GeoData(**raw_geo_data)
    
    assert gd.__dict__ == expectation


@pytest.mark.asyncio
@pytest.mark.unit
async def test_successfull_db_connection(create_mongo_db: MongoClient):
    
    res: dict[str, float] = await create_mongo_db.client.admin.command('ping')
    assert res == {'ok': 1.0}


@pytest.mark.asyncio
@pytest.mark.unit
async def test_insert(create_mongo_db: MongoClient):
    index: int = 100

    res = await create_mongo_db.do_insert(data=[{f"i": f"i"} for i in range(index)])
    assert res == True

@pytest.mark.asyncio
@pytest.mark.unit
async def test_find_one(create_mongo_db: MongoClient):

    res: dict = await create_mongo_db.do_find_one(k=f"{1}", v=f"{1}")
    print(res)
    assert res.get('1') == '1'


@pytest.mark.asyncio
@pytest.mark.parametrize('coordinates', geo_data_list)
@pytest.mark.functional
async def test(coordinates: list[tuple[int, int, str, dict[str, str], dict[str, str]]], create_mongo_db: MongoClient):

    latitude, longitude, expectation_code, raw_geo_data, expectation = coordinates
    gdm: GeoDataMessage = GeoDataMessage(lat=latitude, long=longitude)
    mock_geolocator: Mock = Mock()
    mock_geolocator.reverse= {
        'address': 'Your fake address data'
    }

    with patch('geopy.geocoders.Nominatim', return_value=mock_geolocator):
        location = await get_location(gdm)
        
        assert location.get('country') == raw_geo_data.get('country')
        assert location.get('city') == raw_geo_data.get('city')
        assert location.get('state') == raw_geo_data.get('state')
        assert location.get('region') == raw_geo_data.get('region')
        assert location.get('iso') == raw_geo_data.get('iso')
        
        country_code: str = location['country_code']
        assert country_code.lower() == expectation_code.lower()

    geo_data: GeoData = GeoData(**location)
    assert geo_data.__dict__ == expectation

    is_insert_success: bool = await create_mongo_db.do_insert(data=[geo_data.__dict__])

    assert is_insert_success == True

    res: dict = await create_mongo_db.do_find_one(k="city", v=geo_data.city)

    assert res.get('city') == geo_data.city
    assert res.get('country') == geo_data.country
    assert res.get('country_code') == geo_data.country_code
    assert res.get('iso') == geo_data.iso
    assert res.get('region') == geo_data.region
    assert res.get('state') == geo_data.state
