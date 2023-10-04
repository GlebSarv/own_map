package save_in_cache

import (
	"encoding/json"
	"fmt"
	"strconv"

	"github.com/bradfitz/gomemcache/memcache"
	"github.com/glebServ/save_iso_code/internal/model"
)

type MemcachedClient struct {
	client *memcache.Client
}

func NewMemcachedClient(url string, port int) *MemcachedClient {
	address := url + ":" + strconv.Itoa(port)
	mc := memcache.New(address)

	return &MemcachedClient{
		client: mc,
	}
}

func (mc *MemcachedClient) SetCache(models map[string]model.Country) error {

	for key, value := range models {

		json_value, err := json.Marshal(value)

		if err != nil {
			fmt.Println(err)
		}

		err = mc.client.Set(&memcache.Item{Key: key, Value: json_value})

		if err != nil {
			fmt.Println(err)
		}
	}

	return nil
}

func (mc *MemcachedClient) GetCache(code string) (model.Country, error) {

	country, err := mc.client.Get(code)
	var retrievedCountry model.Country

	if err != nil {
		return retrievedCountry, err
	}

	err = json.Unmarshal(country.Value, &retrievedCountry)

	if err != nil {
		return retrievedCountry, err
	}

	return retrievedCountry, nil
}
