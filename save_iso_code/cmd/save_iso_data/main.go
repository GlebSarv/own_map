package main

import (
	"fmt"
	"log"

	config "github.com/glebServ/save_iso_code/internal/configuration"
	"github.com/glebServ/save_iso_code/internal/download_data"
	"github.com/glebServ/save_iso_code/internal/save_in_cache"
)

func main() {

	config := config.NewConfig()

	mc := save_in_cache.NewMemcachedClient(config.Memcache.Address, config.Memcache.Port)
	dd := download_data.DownloadData()

	mc.SetCache(dd)
	country, err := mc.GetCache("RU")

	if err != nil {
		log.Println(country, " is not country code")
	}

	fmt.Println(country)
}
