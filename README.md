Jako odpowiedzialna osoba studencka zwracasz książki na czas, gdy już ich nie potrzebujesz. Niemniej jednak wypożyczając wiele książek, musisz co kilka dni jakąś przedłużyć.

![books](books.jpg "Books" | width=200)

Ten prosty program będzie przedłużał książki za Ciebie 4 dni przed upływem terminu. Dlaczego 4 dni? Ponieważ 3 dni przed upływem terminu wysyłane jest powiadomienie mailowe - jeśli je dostaniesz, to znaczy, że coś poszło nie tak.

## Jak to działa?

Program uruchamia się co 6 godzin, loguje na Twoje konto biblioteczne, sprawdza terminy zwrotu wypożyczonych książek i przedłuża te, których czas już nadszedł.

## Instalacja

Zaloguj się na serwer ([np. na serwer uniwersytecki (tylko primus!)](https://www.fuw.edu.pl/~kpias/pzfmni/instrukcja_login_to_OKWF.pdf#subsubsection.5.1.1)) i uruchom poniższy kod.

```shell
cd ~ && curl -o- https://raw.githubusercontent.com/krolikbrunatny/buwu/main/install.sh | bash
```

Skrypt poprosi Cię o podanie numeru karty bibliotecznej oraz hasła, następnie utworzy katalog `buwu` wraz z potrzebną konfiguracją, a na koniec ustawi automatyczne uruchamianie się programu co 6 godzin.

## Jak wyłączyć?

Aby zakazać programowi samodzielnego uruchamiania:

```shell
(crontab -l 2>/dev/null | grep -v "buwu/buwu") | crontab -
```

## Ważna uwaga

Twój numer karty bibliotecznej i hasło są przechowywane w pliku `buwu/config.toml` w postaci jawnej (**plain text**), ale dostęp do tego pliku masz tylko Ty.

Jeśli ktoś uzyska dostęp do Twojego konta bibliotecznego, musiał być to administrator systemu lub system został "skompromitowany" - w takiej sytuacji należy poinformować administratora.

Pamiętaj, aby zawsze używać unikalnych haseł dla różnych serwisów internetowych.
