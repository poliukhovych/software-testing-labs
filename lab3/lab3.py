import unittest
from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC
from selenium.webdriver.chrome.service import Service
from webdriver_manager.chrome import ChromeDriverManager

BASE_URL = "https://senior-pomidor.com.ua"


class SeniorPomidorSmokeTests(unittest.TestCase):
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

    def test_home_page_opens(self):
        self.driver.get(BASE_URL)
        self.assertIn("Сеньйор", self.driver.title)
        self.wait.until(EC.presence_of_element_located((By.LINK_TEXT, "Прайс-лист")))

    def test_category_page_opens(self):
        self.driver.get(BASE_URL)
        self.driver.find_element(By.PARTIAL_LINK_TEXT, "Насіння овочів").click()
        self.wait.until(EC.url_contains("semena_ovoshei"))
        self.assertIn("Насіння овочів", self.driver.page_source)

    def test_cart_page_opens(self):
        self.driver.get(BASE_URL)
        self.driver.find_element(By.PARTIAL_LINK_TEXT, "ОФОРМИТИ ЗАМОВЛЕННЯ").click()
        self.wait.until(EC.url_contains("user.php"))
        self.assertIn("Ваш кошик порожній", self.driver.page_source)

    def test_product_card_opens(self):
        self.driver.get(BASE_URL + "/1/1/semena_ovoshei/4/dynja.html")
        self.driver.find_element(By.PARTIAL_LINK_TEXT, "насіння диня Алушта 3 г").click()
        self.wait.until(EC.presence_of_element_located((By.TAG_NAME, "h1")))
        self.assertIn("насіння диня Алушта 3 г", self.driver.page_source)
        self.assertIn("14,50 грн", self.driver.page_source)


if __name__ == "__main__":
    unittest.main()
