package config

import (
	"log"

	"github.com/spf13/viper"
)

type Config struct {
	Memcache *MemcachedConfig
}

type MemcachedConfig struct {
	Port    int
	Address string
}

func NewConfig() *Config {
	viper.SetConfigFile(".env")
	err := viper.ReadInConfig()

	if err != nil {
		log.Fatalf("Cannot read configuration file")
	}

	mc_address := viper.GetString("MEMCACHED_URI")
	mc_port := viper.GetInt("MEMCACHED_PORT")

	mc := newMemcachedConfig(mc_address, mc_port)

	return &Config{
		Memcache: mc,
	}
}

func newMemcachedConfig(address string, port int) *MemcachedConfig {

	return &MemcachedConfig{
		Port:    port,
		Address: address,
	}
}
