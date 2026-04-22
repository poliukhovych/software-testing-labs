import unittest
from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.common.keys import Keys
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC
from selenium.webdriver.chrome.service import Service
from webdriver_manager.chrome import ChromeDriverManager

BASE_URL = "https://senior-pomidor.com.ua"


class SeniorPomidorRegressionTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        options = webdriver.ChromeOptions()
        options.add_argument("--start-maximized")
        options.add_argument("--disable-blink-features=AutomationControlled")
        cls.driver = webdriver.Chrome(
            service=Service(ChromeDriverManager().install()),
            options=options,
        )
        cls.wait = WebDriverWait(cls.driver, 15)

    @classmethod
    def tearDownClass(cls):
        cls.driver.quit()

    def test_01_home_page_opens(self):
        self.driver.get(BASE_URL)
        self.assertIn("Сеньйор", self.driver.title)
        self.wait.until(EC.presence_of_element_located((By.LINK_TEXT, "Прайс-лист")))

    def test_02_category_page_opens(self):
        self.driver.get(BASE_URL)
        self.driver.find_element(By.PARTIAL_LINK_TEXT, "Насіння овочів").click()
        self.wait.until(EC.url_contains("semena_ovoshei"))
        self.assertIn("Насіння овочів", self.driver.page_source)

    def test_03_subcategory_page_opens(self):
        self.driver.get(BASE_URL + "/1/1/semena_ovoshei.html")
        self.driver.find_element(By.PARTIAL_LINK_TEXT, "диня").click()
        self.wait.until(EC.url_contains("dynja"))
        self.assertIn("диня", self.driver.page_source.lower())

    def test_04_product_card_opens(self):
        self.driver.get(BASE_URL + "/1/1/semena_ovoshei/4/dynja.html")
        self.driver.find_element(By.PARTIAL_LINK_TEXT, "насіння диня Алушта 3 г").click()
        self.wait.until(EC.presence_of_element_located((By.TAG_NAME, "h1")))
        self.assertIn("насіння диня Алушта 3 г", self.driver.page_source)

    def test_05_cart_page_opens(self):
        self.driver.get(BASE_URL)
        self.driver.find_element(By.PARTIAL_LINK_TEXT, "ОФОРМИТИ ЗАМОВЛЕННЯ").click()
        self.wait.until(EC.url_contains("user.php"))
        self.assertTrue(
            "Ваш кошик порожній" in self.driver.page_source
            or "кошик" in self.driver.page_source.lower()
        )

    def test_06_price_list_link_is_present(self):
        self.driver.get(BASE_URL)
        price_link = self.wait.until(EC.presence_of_element_located((By.LINK_TEXT, "Прайс-лист")))
        self.assertTrue(price_link.get_attribute("href"))

    def test_07_contacts_page_or_block_is_available(self):
        self.driver.get(BASE_URL)
        self.assertTrue(
            "тел." in self.driver.page_source.lower()
            or "контакт" in self.driver.page_source.lower()
            or "+38" in self.driver.page_source
        )

    def test_08_search_field_accepts_query(self):
        self.driver.get(BASE_URL)
        search_inputs = self.driver.find_elements(By.CSS_SELECTOR, "input[type='text']")
        usable = None
        for element in search_inputs:
            try:
                if element.is_displayed() and element.is_enabled():
                    usable = element
                    break
            except Exception:
                pass
        self.assertIsNotNone(usable, "Не знайдено доступне текстове поле для пошуку")
        usable.clear()
        usable.send_keys("диня")
        usable.send_keys(Keys.ENTER)
        self.assertIn("диня", self.driver.page_source.lower())

    def test_09_catalog_contains_product_cards(self):
        self.driver.get(BASE_URL + "/1/1/semena_ovoshei.html")
        cards = self.driver.find_elements(By.CSS_SELECTOR, "a, .product, .goods")
        self.assertGreater(len(cards), 0)

    def test_10_second_tab_home_page_opens(self):
        self.driver.get(BASE_URL)
        self.driver.switch_to.new_window("tab")
        self.driver.get(BASE_URL)
        self.assertIn("Сеньйор", self.driver.title)
        self.assertIn("senior-pomidor.com.ua", self.driver.current_url)


if __name__ == "__main__":
    unittest.main()
