# Приложение для генерации ссылок на архивы дайком в облаке яндекс
# Эмулятор рабочего приложения

Логика взаимодействия приложения с внешними системами:
- Приемник DICOM получает исследования запросом c-store от стороннего PACS и генерирует архив в формате .iso сохраняя результат в S3 хранилище yandex.cloud
- Сторонняя медицинская  система обращается в приложение REST к методу GET /link за ссылкой по идентификатору исследования либо по номеру назначения. REST приложение генерирует ссылку на скачивание архива с ограниченным временем жизни, передает ответом в медицинскую систему
- Медицинская система может передавать данную ссылку как другим приложениям так и использовать  самостоятельно

* приемник REST доступен по адресу http://84.201.160.105:8000
* приемник DICOM c-store доступен 84.201.160.105:11111
* Swagger REST доступен по адресу http://84.201.160.105:8000/swagger-ui/#/crate/link_study