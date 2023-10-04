package download_data

import (
	"log"
	"net/http"
	"strings"

	"github.com/PuerkitoBio/goquery"
	"github.com/glebServ/save_iso_code/internal/model"
)

const RU_URL = "https://ru.wikipedia.org/wiki/ISO_3166-1"
const EN_URL = "https://en.wikipedia.org/wiki/ISO_3166-1"

func get_page(url string) *goquery.Document {

	res, err := http.Get(url)
	if err != nil {
		log.Fatal(err)
	}

	defer res.Body.Close()
	if res.StatusCode != 200 {
		log.Fatalf("status code error %d %s", res.StatusCode, res.Status)
	}

	doc, err := goquery.NewDocumentFromReader(res.Body)
	if err != nil {
		log.Fatal(err)
	}

	return doc
}

func get_ru_data(doc *goquery.Document, countries map[string]model.Country) {

	tBody := doc.Find("tbody").First()
	title := true
	tBody.Find("tr").Each(func(i int, element *goquery.Selection) {
		if title {
			title = false
		} else {
			lines := strings.Split(element.Text(), "\n")
			iso := strings.TrimSpace(lines[2])
			countries[iso] = model.Country{
				NameRu:    strings.TrimSpace(lines[1]),
				Iso:       strings.TrimSpace(lines[2]),
				Iso_3:     strings.TrimSpace(lines[3]),
				Iso_digit: strings.TrimSpace(lines[4]),
			}
		}
	})
}

func get_en_data(doc *goquery.Document, countries map[string]model.Country) {

	tbody := doc.Find("tbody").Eq(1)

	tbody.Find("tr").Each(func(i int, element *goquery.Selection) {

		lines := strings.Split(element.Text(), "\n")
		iso := strings.TrimSpace(lines[3])
		country, ok := countries[iso]

		if ok {
			country.NameEn = strings.TrimSpace(lines[1])
			countries[iso] = country
		}

	})
}

func DownloadData() map[string]model.Country {

	countries := make(map[string]model.Country)
	go func() {
		doc := get_page(RU_URL)
		get_ru_data(doc, countries)
	}()
	go func() {
		doc := get_page(EN_URL)
		get_en_data(doc, countries)
	}()

	return countries
}
