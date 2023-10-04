package save_in_cache

import (
	"testing"

	"github.com/glebServ/save_iso_code/internal/model"
	"github.com/stretchr/testify/assert"
)

func TestSmthng_test(t *testing.T) {

	mc := NewMemcachedClient("0.0.0.0", 11211)

	country := make(map[string]model.Country)
	country["PE"] = model.Country{
		NameRu:    "Перу",
		NameEn:    "Peru",
		Iso:       "PE",
		Iso_3:     "PER",
		Iso_digit: "604",
	}

	mc.SetCache(country)

	c, err := mc.GetCache("PEQ")

	if err != nil {
		return
	}

	assert.Equal(t, c, country["PE"], "equal")
}
