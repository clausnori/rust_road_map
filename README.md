
## Этап 1: Основы и начало работы

### Установка и настройка
- [ ] [[Установить Rust через rustup]]
- [ ] [[Настроить редактор кода (VS Code с rust-analyzer или другой)]]
- [ ] [[Изучить основные команды cargo]]
- [ ] [[Понимание toolchain (stable, beta, nightly)]]
- [ ] [[Настройка clippy и rustfmt]]

### Основы синтаксиса
 - [ ] [[fn main(){}]]
#### Переменные и mutability
- [ ] [[Объявление переменных с `let`]]
- [ ]  [[println!() и dbg!()]]
- [ ] [[Понимание неизменяемости по умолчанию]]
- [ ] [[Использование `mut` для изменяемых переменных]]
- [ ] [[Переназначение переменных (shadowing)]]
- [ ] [[Различия между shadowing и мутабельностью]]
- [ ] Область видимости переменных
- [ ] Правила именования переменных (snake_case)
- [ ] **Практика**: создать программу с примерами shadowing и mut

#### Константы и статические переменные
- [ ] Объявление констант с `const`
- [ ] Требования к константам (compile-time вычисления)
- [ ] Статические переменные с `static`
- [ ] Различия между `const` и `static`
- [ ] Мутабельные статические переменные и `unsafe`
- [ ] Соглашения по именованию (SCREAMING_SNAKE_CASE)
- [ ] Использование констант в pattern matching
- [ ] **Практика**: []

#### Комментарии и документация
- [ ] Обычные комментарии (`//` и `/* */`)
- [ ] Документационные комментарии (`///` и `/** */`)
- [ ] Внутренние док-комментарии (`//!` и `/*! */`)
- [ ] Markdown в документации
- [ ] Примеры кода в документации
- [ ] Команда `cargo doc`

## Этап 2: Типы данных

### Скалярные типы

#### Целочисленные типы
- [ ] Знаковые типы (i8, i16, i32, i64, i128, isize)
- [ ] Беззнаковые типы (u8, u16, u32, u64, u128, usize)
- [ ] Архитектурно-зависимые типы (isize, usize)
- [ ] Диапазоны значений для каждого типа
- [ ] Переполнение целых чисел в debug vs release
- [ ] Wrapping, checked, saturating, overflowing арифметика
- [ ] Литералы с префиксами (0x для hex, 0o для octal, 0b для binary)
- [ ] Разделители в числах (1_000_000)
- [ ] **Практика**: []

#### Методы для целочисленных типов
- [ ] Арифметические методы (abs(), pow(), sqrt())
- [ ] Методы проверки переполнения (checked_*, wrapping_*, saturating_*, overflowing_*)
- [ ] Битовые операции (count_ones(), leading_zeros(), trailing_zeros())
- [ ] Методы преобразования (as, try_into(), into())
- [ ] Методы работы с байтами (to_be_bytes(), from_le_bytes(), to_ne_bytes())
- [ ] Методы сравнения (min(), max(), clamp())
- [ ] Методы для работы с степенями двойки (is_power_of_two(), next_power_of_two())

#### Типы с плавающей точкой
- [ ] f32 и f64 типы
- [ ] Стандарт IEEE 754
- [ ] Точность и диапазон f32 vs f64
- [ ] Специальные значения (NaN, INFINITY, NEG_INFINITY, NEG_ZERO)
- [ ] Проблемы сравнения чисел с плавающей точкой
- [ ] Проблемы точности при вычислениях
- [ ] Научная нотация в литералах (1e10, 2.5e-4)
- [ ] Выбор между f32 и f64 в различных сценариях
- [ ] **Практика**: программа для сравнения точности вычислений

#### Методы для типов с плавающей точкой
- [ ] Тригонометрические функции (sin(), cos(), tan(), asin(), acos(), atan(), atan2())
- [ ] Гиперболические функции (sinh(), cosh(), tanh())
- [ ] Логарифмические функции (ln(), log10(), log2(), ln_1p(), log10())
- [ ] Экспоненциальные функции (exp(), exp2(), exp_m1())
- [ ] Степенные функции (powf(), sqrt(), cbrt(), hypot())
- [ ] Функции округления (round(), floor(), ceil(), trunc(), fract())
- [ ] Методы проверки (is_nan(), is_finite(), is_infinite(), is_normal(), is_subnormal())
- [ ] Методы сравнения (min(), max(), partial_cmp())
- [ ] Функции для работы с знаком (abs(), signum(), copysign())
- [ ] Методы разбора (to_bits(), from_bits())

#### Логический тип bool
- [ ] Значения true и false
- [ ] Логические операторы (&&, ||, !)
- [ ] Короткое замыкание (short-circuiting)
- [ ] Использование в условных выражениях
- [ ] Преобразования из других типов (недоступны напрямую)
- [ ] bool в pattern matching
- [ ] Размер bool в памяти (1 байт, но alignment может быть больше)
- [ ] **Практика**: логические схемы и таблицы истинности

#### Символьный тип char
- [ ] Unicode и char в Rust (4 байта, Unicode Scalar Value)
- [ ] Различия с char в других языках
- [ ] Литералы символов ('a', 'ж', '🦀')
- [ ] Escape-последовательности ('\n', '\t', '\r', '\\', '\'', '\"')
- [ ] Unicode escape-последовательности ('\u{1F980}', '\u{041F}')
- [ ] Hexadecimal escape-последовательности ('\x41')
- [ ] Методы char (is_alphabetic(), is_numeric(), to_uppercase(), to_lowercase())
- [ ] Итерация по кодовым точкам
- [ ] **Практика**: анализатор текста с подсчетом различных типов символов

### Составные типы

#### Кортежи (Tuples)
- [ ] Создание кортежей разных размеров
- [ ] Типы кортежей (T1, T2, T3, ...)
- [ ] Деструктуризация кортежей (let (x, y, z) = tuple)
- [ ] Доступ к элементам через индексы (tuple.0, tuple.1)
- [ ] Кортежи как возвращаемые значения функций
- [ ] Unit type () как пустой кортеж
- [ ] Вложенные кортежи
- [ ] Кортежи с одним элементом (trailing comma)
- [ ] **Практика**: функции, возвращающие множественные значения

#### Массивы (Arrays)
- [ ] Создание массивов фиксированного размера [T; N]
- [ ] Инициализация массивов ([1, 2, 3] vs [0; 10])
- [ ] Доступ к элементам с проверкой границ
- [ ] Panic при выходе за границы
- [ ] Получение длины с .len()
- [ ] Итерация по массивам (.iter(), .iter_mut(), .into_iter())
- [ ] Методы массивов (map(), filter(), contains())
- [ ] Многомерные массивы [[T; M]; N]
- [ ] **Практика**: матричные вычисления с массивами

#### Срезы (Slices)
- [ ] Понятие среза &[T]
- [ ] Создание срезов из массивов и векторов
- [ ] Синтаксис диапазонов (0..5, 1.., ..3, ..)
- [ ] Мутабельные срезы &mut [T]
- [ ] Методы срезов (len(), is_empty(), first(), last())
- [ ] Методы поиска (binary_search(), contains())
- [ ] Методы сортировки (sort(), sort_by(), sort_unstable())
- [ ] Методы разбиения (split(), chunks(), windows())
- [ ] **Практика**: алгоритмы сортировки и поиска

## Этап 3: Управление потоком

### Условные выражения

#### if выражения
- [ ] Базовый синтаксис if
- [ ] if-else конструкции
- [ ] else if цепочки
- [ ] if как выражение (возвращающее значение)
- [ ] Условия должны быть bool (нет автоконверсии)
- [ ] Вложенные if выражения
- [ ] **Практика**: []

#### let-else выражения (Rust 1.65+)
- [ ] Синтаксис let-else
- [ ] Использование с Result и Option
- [ ] Альтернатива unwrap() для раннего возврата
- [ ] **Практика**: []

### Циклы

#### loop - бесконечные циклы
- [ ] Синтаксис loop
- [ ] Выход из цикла с break
- [ ] Пропуск итерации с continue
- [ ] Возвращение значений из loop с break value
- [ ] Метки циклов для вложенных loop
- [ ] **Практика**: []

#### while циклы
- [ ] Синтаксис while с условием
- [ ] while true vs loop
- [ ] Использование break и continue
- [ ] **Практика**: []

#### for циклы
- [ ] for с диапазонами (0..10, 0..=10)
- [ ] for с итераторами (.iter(), .iter_mut(), .into_iter())
- [ ] for с коллекциями
- [ ] Деструктуризация в for циклах
- [ ] for с enumerate() для получения индексов
- [ ] **Практика**: []

#### Продвинутые возможности циклов
- [ ] Метки циклов ('outer: loop)
- [ ] break и continue с метками
- [ ] Возвращение значений из именованных циклов
- [ ] **Практика**: сложные алгоритмы с вложенными циклами

### Сопоставление с образцом

#### match выражения
- [ ] Базовый синтаксис match
- [ ] Исчерпывающность паттернов
- [ ] Wildcard паттерн _
- [ ] match как выражение
- [ ] Множественные паттерны с |
- [ ] **Практика**: простой калькулятор

#### Паттерны в match
- [ ] Литеральные паттерны
- [ ] Переменные в паттернах
- [ ] Структурированные паттерны
- [ ] Диапазоны в паттернах (1..=5)
- [ ] Игнорирование значений с _
- [ ] **Практика**: парсер простых команд

## Этап 4: Функции

### Основы функций
- [ ] Объявление функций с fn
- [ ] Правила именования (snake_case)
- [ ] Параметры функций с обязательными типами
- [ ] Возвращаемые типы
- [ ] Возврат значений (с return и без)
- [ ] Unit type () как возвращаемый тип по умолчанию

### Параметры и аргументы
- [ ] Передача по значению vs по ссылке
- [ ] Мутабельные параметры (&mut)
- [ ] Множественные параметры
- [ ] Параметры-кортежи
- [ ] Деструктуризация параметров
- [ ] **Практика**: библиотека математических функций

### Выражения vs утверждения
- [ ] Понимание различий между expressions и statements
- [ ] Утверждения заканчиваются точкой с запятой
- [ ] Выражения возвращают значения
- [ ] Последнее выражение как возвращаемое значение
- [ ] Блоки как выражения
- [ ] Преобразование выражения в утверждение
- [ ] **Практика**: функции в функциональном стиле

### Область видимости и блоки кода
- [ ] Синтаксис блоков {}
- [ ] Блоки как выражения
- [ ] Возвращение значений из блоков
- [ ] Область видимости в блоках
- [ ] Вложенные блоки и shadowing
- [ ] Блоки в условных выражениях и циклах
- [ ] **Практика**: сложные вычисления с промежуточными блоками

## Этап 5: Владение памятью (Ownership) - КЛЮЧЕВОЙ ЭТАП

### Концепция владения
- [ ] Понятие владения (ownership) - уникальность Rust
- [ ] Три правила владения
- [ ] Область видимости (scope) и автоматическое освобождение
- [ ] Функция drop и RAII (Resource Acquisition Is Initialization)
- [ ] Stack vs Heap память
- [ ] **Практика**: примеры с визуализацией владения

### Move семантика
- [ ] Перемещение значений при присваивании
- [ ] Перемещение при передаче в функции
- [ ] Перемещение при возврате из функций
- [ ] Copy trait для простых типов
- [ ] Clone trait для глубокого копирования
- [ ] Различия между Copy и Clone
- [ ] Типы, реализующие Copy автоматически
- [ ] **Практика**: работа со строками и векторами

### Заимствование (Borrowing)
- [ ] Неизменяемые ссылки (&T)
- [ ] Изменяемые ссылки (&mut T)
- [ ] Правила заимствования (borrow checker)
- [ ] Одновременно только одна мутабельная ссылка
- [ ] Неограниченное количество неизменяемых ссылок
- [ ] Нет одновременных мутабельных и неизменяемых ссылок
- [ ] Висячие ссылки (dangling references) - невозможны
- [ ] **Практика**: функции для обработки данных без перемещения

### Времена жизни (Lifetimes)
- [ ] Концепция времен жизни
- [ ] Автоматический вывод времен жизни (lifetime elision)
- [ ] Явные аннотации времен жизни ('a, 'b)
- [ ] Времена жизни в функциях
- [ ] Времена жизни в структурах
- [ ] Статическое время жизни 'static
- [ ] **Практика**: функции, возвращающие ссылки

### Срезы как заимствования
- [ ] Строковые срезы (&str) как заимствования String
- [ ] Срезы массивов (&[T]) как заимствования массивов/векторов
- [ ] Создание срезов с диапазонами
- [ ] Мутабельные срезы (&mut [T])
- [ ] **Практика**: функции для работы со срезами строк

## Этап 6: Коллекции

### Строки (Strings)
- [ ] Различия между String и &str
- [ ] UTF-8 encoding в Rust
- [ ] Создание строк (String::new(), String::from(), .to_string())
- [ ] Модификация строк (push(), push_str(), insert(), remove())
- [ ] Конкатенация строк (+, format!, write!)
- [ ] Индексирование строк (почему невозможно)
- [ ] Итерация по символам (.chars()) и байтам (.bytes())
- [ ] Срезы строк и границы символов
- [ ] Методы поиска (contains(), find(), starts_with(), ends_with())
- [ ] Методы замены (replace(), replacen())
- [ ] Методы преобразования (to_uppercase(), to_lowercase(), trim())
- [ ] Разбиение строк (split(), lines(), split_whitespace())
- [ ] **Практика**: текстовый процессор

### Векторы (Vec<T>)
- [ ] Создание векторов (Vec::new(), vec! макрос)
- [ ] Добавление элементов (push(), insert(), extend())
- [ ] Удаление элементов (pop(), remove(), clear(), truncate())
- [ ] Доступ к элементам ([], get(), get_mut())
- [ ] Итерация по векторам (.iter(), .iter_mut(), .into_iter())
- [ ] Capacity vs length (len(), capacity(), reserve(), shrink_to_fit())
- [ ] Методы поиска (contains(), binary_search())
- [ ] Методы сортировки (sort(), sort_by(), sort_unstable())
- [ ] Деструктуризация векторов
- [ ] **Практика**: динамический список задач

### HashMap<K, V>
- [ ] Создание HashMap (HashMap::new(), collect())
- [ ] Вставка значений (insert(), entry().or_insert())
- [ ] Получение значений (get(), [], get_mut())
- [ ] Удаление значений (remove())
- [ ] Проверка существования ключей (contains_key())
- [ ] Итерация по ключам, значениям и парам
- [ ] Entry API (entry(), or_insert(), or_insert_with(), and_modify())
- [ ] Хеш-функции и Eq trait
- [ ] **Практика**: счетчик слов в тексте

### Другие важные коллекции
- [ ] HashSet<T> для уникальных значений
- [ ] VecDeque<T> для двусторонней очереди
- [ ] BTreeMap<K, V> для упорядоченных ассоциативных массивов
- [ ] BTreeSet<T> для упорядоченных уникальных значений
- [ ] LinkedList<T> (редко используется)
- [ ] **Практика**: выбор подходящей коллекции для задач

## Этап 7: Структуры и методы

### Определение структур
- [ ] Обычные структуры (struct с именованными полями)
- [ ] Tuple структуры (struct с неименованными полями)
- [ ] Unit структуры (struct без полей)
- [ ] Инициализация структур
- [ ] Shorthand синтаксис полей
- [ ] Оператор обновления структур (..)
- [ ] **Практика**: моделирование предметной области

### Методы и ассоциированные функции
- [ ] Блоки impl для структур
- [ ] Методы экземпляра (&self)
- [ ] Мутирующие методы (&mut self)
- [ ] Методы-потребители (self)
- [ ] Ассоциированные функции (без self)
- [ ] Конструкторы как ассоциированные функции
- [ ] Множественные блоки impl
- [ ] **Практика**: структура Rectangle с методами

### Видимость полей и методов
- [ ] Приватные поля по умолчанию
- [ ] Публичные поля с pub
- [ ] Публичные методы с pub
- [ ] Инкапсуляция в Rust
- [ ] **Практика**: API дизайн с правильной инкапсуляцией

## Этап 8: Перечисления и продвинутый паттерн матчинг

### Определение перечислений
- [ ] Простые перечисления (enum без данных)
- [ ] Перечисления с данными (различные типы для каждого варианта)
- [ ] Перечисления со структурными данными
- [ ] Методы для enums через impl блоки
- [ ] **Практика**: моделирование состояний программы

### Важные стандартные перечисления
- [ ] Option<T> - обработка отсутствующих значений
- [ ] Some(T) и None варианты
- [ ] Методы: unwrap(), expect(), is_some(), is_none()
- [ ] Методы: map(), and_then(), or(), or_else()
- [ ] Комбинирование Option значений
- [ ] Result<T, E> - обработка ошибок
- [ ] Ok(T) и Err(E) варианты
- [ ] Методы: unwrap(), expect(), is_ok(), is_err()
- [ ] Методы: map(), map_err(), and_then(), or_else()
- [ ] Оператор ? для распространения ошибок
- [ ] **Практика**: безопасная обработка пользовательского ввода

### Продвинутый паттерн матчинг
- [ ] Деструктуризация структур в match
- [ ] Деструктуризация кортежей и массивов
- [ ] Паттерн guards (if условия в match)
- [ ] Binding с @ символом
- [ ] Nested паттерны
- [ ] Диапазоны в паттернах (1..=5, 'a'..='z')
- [ ] **Практика**: парсер JSON-подобных структур

### if let и while let
- [ ] if let для простых случаев match
- [ ] while let для циклов с паттернами
- [ ] Комбинирование с else
- [ ] **Практика**: обработка потоков данных

### Паттерны в других местах
- [ ] Паттерны в let утверждениях
- [ ] Паттерны в параметрах функций
- [ ] Паттерны в for циклах
- [ ] Опровержимые vs неопровержимые паттерны
- [ ] **Практика**: элегантная деструктуризация данных

## Этап 9: Обработка ошибок

### Философия обработки ошибок в Rust
- [ ] Ошибки как часть типа (Result<T, E>)
- [ ] Принуждение к явной обработке ошибок
- [ ] Различие между recoverable и unrecoverable ошибками

### Unrecoverable ошибки - panic!
- [ ] Макрос panic! и его использование
- [ ] Автоматические panic (array bounds, integer overflow)
- [ ] Stack unwinding vs abort on panic
- [ ] Обработка panic в тестах
- [ ] std::panic::catch_unwind (не рекомендуется для обычного кода)
- [ ] **Практика**: отладка с помощью panic

### Recoverable ошибки с Result<T, E>
- [ ] Создание Result значений
- [ ] Обработка с match
- [ ] Методы unwrap() и expect() (осторожно!)
- [ ] Безопасные методы: unwrap_or(), unwrap_or_else()
- [ ] Метод unwrap_or_default()
- [ ] **Практика**: чтение файлов с обработкой ошибок

### Распространение ошибок
- [ ] Оператор ? для автоматического распространения
- [ ] ? с Option<T>
- [ ] ? с Result<T, E>
- [ ] Возврат Result из main функции
- [ ] Преобразования типов ошибок с From trait
- [ ] **Практика**: цепочки операций с ошибками

### Создание кастомных типов ошибок
- [ ] Реализация std::error::Error trait
- [ ] Реализация Display и Debug traits
- [ ] Создание enum для различных типов ошибок
- [ ] Использование thiserror crate (концептуально)
- [ ] **Практика**: библиотека с полноценной обработкой ошибок

### Комбинирование Option и Result
- [ ] Преобразование между Option и Result
- [ ] Методы ok_or() и ok_or_else()
- [ ] Комбинирование в цепочки вызовов
- [ ] **Практика**: валидация пользовательских данных

## Этап 10: Тестирование

### Unit тесты
- [ ] Атрибут #[cfg(test)]
- [ ] Функции с #[test]
- [ ] Макросы assert!, assert_eq!, assert_ne!
- [ ] Тестирование panic с #[should_panic]
- [ ] Возвращение Result в тестах

### Интеграционные тесты
- [ ] Папка tests/
- [ ] Организация интеграционных тестов
- [ ] Общий код для тестов

### Организация тестов
- [ ] Запуск тестов с cargo test
- [ ] Фильтрация тестов
- [ ] Игнорирование тестов с #[ignore]
- [ ] Параллельное выполнение Тестирование

## Этап 11: Обобщенное программирование (Generics)

### Основы обобщений
- [ ] Мотивация для generics (избежание дублирования кода)
- [ ] Обобщенные функции
- [ ] Синтаксис <T> для типовых параметров
- [ ] Множественные типовые параметры <T, U>
- [ ] Соглашения по именованию (T, U, V...)
- [ ] **Практика**: обобщенные функции поиска

### Обобщенные структуры
- [ ] Структуры с типовыми параметрами
- [ ] Методы для обобщенных структур
- [ ] Ограничения области действия типовых параметров
- [ ] **Практика**: обобщенная структура Point<T>

### Обобщенные перечисления
- [ ] Перечисления с типовыми параметрами
- [ ] Option<T> и Result<T, E> как примеры
- [ ] **Практика**: создание собственного Result-подобного типа

### Обобщенные методы
- [ ] Методы с собственными типовыми параметрами
- [ ] Комбинирование с типовыми параметрами структуры
- [ ] **Практика**: методы преобразования типов

### Производительность generics
- [ ] Мономорфизация (monomorphization)
- [ ] Zero-cost abstractions
- [ ] Размер скомпилированного кода

## Этап 12: Трейты (Traits) - КЛЮЧЕВОЙ ЭТАП

### Определение и реализация трейтов
- [ ] Синтаксис определения трейтов
- [ ] Методы в трейтах (с реализацией и без)
- [ ] Реализация трейтов для типов (impl Trait for Type)
- [ ] Orphan rule (правило сирот)
- [ ] **Практика**: трейт для геометрических фигур

### Трейты как параметры
- [ ] Параметры типа impl Trait
- [ ] Обобщенные функции с trait bounds
- [ ] Синтаксис where для сложных bounds
- [ ] Множественные trait bounds (T: Clone + Debug)
- [ ] **Практика**: функции с trait bounds

### Возвращение типов с трейтами
- [ ] Возврат impl Trait из функций
- [ ] Ограничения на impl Trait в возвращаемых типах
- [ ] **Практика**: фабричные функции

### Важные стандартные трейты

#### Clone и Copy
- [ ] Clone trait для явного копирования
- [ ] Copy trait для неявного копирования
- [ ] Различия и ограничения Copy
- [ ] Автоматическая реализация с #[derive]

#### Debug и Display
- [ ] Debug для отладочного вывода
- [ ] Display для пользовательского вывода
- [ ] Форматирование с {:?} и {}
- [ ] Кастомные реализации

#### PartialEq и Eq
- [ ] PartialEq для частичного равенства
- [ ] Eq для полного равенства
- [ ] Автоматическая реализация с #[derive]

#### PartialOrd и Ord
- [ ] PartialOrd для частичного упорядочивания
- [ ] Ord for полного упорядочивания
- [ ] Связь с Eq trait

#### Default
- [ ] Default для значений по умолчанию
- [ ] Использование в конструкторах
- [ ] **Практика**: структуры с разумными умолчаниями

### Трейт объекты (Trait Objects)
- [ ] Динамическая диспетчеризация с dyn Trait
- [ ] Ограничения trait objects (object-safe traits)
- [ ] Box<dyn Trait> для ownership
- [ ] &dyn Trait для заимствования
- [ ] **Практика**: коллекции разнотипных объектов

### Продвинутые возможности трейтов
- [ ] Ассоциированные типы
- [ ] Default implementations в train 
- [ ] Supertrain (Наследование в трейтах)
- [ ] Blanket implementation 
- [ ] **Практика** Комплексная иерархия трейтов

## Этап 13: Итераторы и замыкания (Closures)

### Итераторы

- [ ] Iterator trait и метод next()
- [ ] .iter(), .iter_mut(), .into_iter() — различия
- [ ] Lazy вычисления
- [ ] Комбинаторы: map(), filter(), take(), skip(), chain(), zip(), enumerate()
- [ ] Сборка: collect(), count(), sum(), fold(), reduce()
- [ ] Специфичные методы: rev(), peekable(), inspect()
- [ ] Создание собственного итератора (impl Iterator)
- [ ] Итераторы по структурам
- [ ] Практика: подсчёт статистики из текстового файла
### Замыкания (Closures)

- [ ] Синтаксис замыканий: |args| { тело }
- [ ] Захват переменных из окружения (by ref, by mut ref, by move)
- [ ] Типы замыканий: Fn, FnMut, FnOnce
- [ ] Использование замыканий в итераторах
- [ ] Передача замыканий в функции
- [ ] Возврат замыканий из функций
- [ ] Практика: фильтр значений по пользовательскому правилу

## Этап 13: Cargo и модули

### Пакеты и крейты
- [ ] Понятие package, crate, module
- [ ] Binary vs library crates
- [ ] Cargo.toml файл и его структура
- [ ] Зависимости в Cargo.toml (dependencies, dev-dependencies, build-dependencies)
- [ ] Семантическое версионирование (semver)
- [ ] Features и conditional compilation
- [ ] Workspaces для мультикрейтовых проектов
- [ ] Cargo.lock файл и его назначение

### Модульная система
- [ ] Объявление модулей с mod
- [ ] Файловая система модулей (mod.rs vs файлы)
- [ ] Пути к элементам модулей (абсолютные и относительные)
- [ ] pub ключевое слово
- [ ] use декларации и их варианты
- [ ] Повторный export с pub use
- [ ] Группировка use импортов
- [ ] self и super в путях модулей
- [ ] extern crate (legacy) vs Rust 2018 edition

### Видимость и инкапсуляция
- [ ] Приватность по умолчанию
- [ ] pub для публичных элементов
- [ ] pub(crate) - видимость в пределах крейта
- [ ] pub(super) - видимость в родительском модуле
- [ ] pub(in path) - видимость в указанном пути
- [ ] Структуры и enum видимость полей
- [ ] Проектирование API с правильной видимостью

## Этап 14: Умные указатели

### Box<T>
- [ ] Хранение данных в куче
- [ ] Рекурсивные типы данных (linked lists, trees)
- [ ] Deref trait и автоматическое разыменование
- [ ] Drop trait и RAII паттерн
- [ ] Box::leak для получения 'static ссылок
- [ ] Box vs Vec<T> выбор подходящего типа
- [ ] Паттерн "newtype" с Box

### Rc<T> и Arc<T>
- [ ] Reference counting концепция
- [ ] Rc<T> для single-threaded shared ownership
- [ ] Arc<T> для multi-threaded shared ownership
- [ ] Weak<T> references для избежания циклических ссылок
- [ ] Rc::clone() vs .clone()
- [ ] Cycle detection и debugging
- [ ] Performance implications of reference counting

### RefCell<T> и внутренняя изменяемость
- [ ] Interior mutability паттерн
- [ ] Borrow checking во время выполнения
- [ ] borrow() и borrow_mut() методы
- [ ] Panic при нарушении borrowing rules
- [ ] Комбинирование Rc<RefCell<T>>
- [ ] Cell<T> для Copy типов
- [ ] OnceCell и LazyCell для ленивой инициализации
- [ ] Сравнение с Mutex<T>

### UnsafeCell<T> и создание собственных умных указателей
- [ ] UnsafeCell как основа для interior mutability
- [ ] Создание собственных умных указателей
- [ ] Реализация Deref и DerefMut
- [ ] Memory layout и представление

## Этап 15: Многопоточность

### Основы потоков
- [ ] Создание потоков с thread::spawn
- [ ] JoinHandle<T> и join() для ожидания завершения
- [ ] thread::Builder для настройки потоков
- [ ] move в замыканиях для передачи ownership
- [ ] Panic handling в потоках
- [ ] thread::current() и thread::ThreadId
- [ ] Именование потоков для отладки

### Передача данных между потоками
- [ ] Каналы (channels) с mpsc модулем
- [ ] Sender<T> и Receiver<T>
- [ ] channel() vs sync_channel()
- [ ] Отправка и получение сообщений
- [ ] try_recv() и recv_timeout()
- [ ] Закрытие каналов и обработка отключения
- [ ] Множественные Sender'ы
- [ ] crossbeam-channel как альтернатива

### Синхронизация и разделяемое состояние
- [ ] Mutex<T> для взаимного исключения
- [ ] Arc<Mutex<T>> паттерн для shared mutable state
- [ ] lock() и try_lock() методы
- [ ] MutexGuard и автоматическое освобождение
- [ ] Deadlocks: причины и способы избежания
- [ ] RwLock<T> для множественных читателей
- [ ] Barrier для синхронизации групп потоков
- [ ] Condvar для ожидания условий

### Sync и Send трейты
- [ ] Понимание Send trait (можно передавать между потоками)
- [ ] Понимание Sync trait (можно безопасно использовать из нескольких потоков)
- [ ] Автоматическая реализация для большинства типов
- [ ] Случаи когда типы не Send/Sync
- [ ] Реализация для собственных типов
- [ ] Phantom data и marker traits

### Атомарные операции
- [ ] std::sync::atomic модуль
- [ ] AtomicBool, AtomicI32, AtomicUsize и другие
- [ ] Memory ordering (Relaxed, Acquire, Release, SeqCst)
- [ ] Compare-and-swap операции
- [ ] Lock-free программирование основы

## Этап 16: Async/Await программирование

### Основы асинхронности
- [ ] async функции и их сигнатуры
- [ ] await ключевое слово и его использование
- [ ] Future trait и его роль
- [ ] Отличия от обычных функций и производительность
- [ ] State machines генерируемые компилятором
- [ ] Pin<T> и Unpin trait

### Async runtime и экосистема
- [ ] Tokio runtime и его настройка
- [ ] async-std как альтернатива
- [ ] Executor'ы и их роль
- [ ] Выполнение async кода с block_on
- [ ] async блоки и их захват переменных
- [ ] LocalSet для !Send futures

### Async в различных контекстах
- [ ] async методы в трейтах (async-trait crate)
- [ ] async замыкания (экспериментально)
- [ ] Обработка ошибок в async коде
- [ ] async итераторы (Stream trait)
- [ ] Таймауты и отмена с tokio::time
- [ ] Async I/O операции

### Concurrency patterns в async
- [ ] join! макрос для параллельного выполнения
- [ ] select! макрос для гонки задач
- [ ] spawn для создания независимых задач
- [ ] JoinHandle в async контексте
- [ ] Каналы в async (tokio::sync)
- [ ] Async Mutex и RwLock

## Этап 17: Паттерны и идиомы

### Расширенный паттерн матчинг
- [ ] Паттерны в let утверждениях
- [ ] Паттерны в параметрах функций
- [ ] Опровержимые и неопровержимые паттерны
- [ ] if let и while let конструкции
- [ ] Паттерн guards с дополнительными условиями
- [ ] Binding с @ оператором
- [ ] Паттерны с диапазонами (range patterns)
- [ ] Slice patterns для массивов и векторов

### Деструктуризация
- [ ] Деструктуризация структур с переименованием
- [ ] Деструктуризация enums с извлечением данных
- [ ] Деструктуризация кортежей и массивов
- [ ] Игнорирование значений с _ и ..
- [ ] Остаточные паттерны с .. (rest patterns)
- [ ] Вложенная деструктуризация
- [ ] Ref и ref mut в паттернах

### Функциональные паттерны
- [ ] Combinators для Option<T> и Result<T, E>
- [ ] Chain методы для читаемого кода
- [ ] Closure захват и различные Fn* traits
- [ ] Higher-order functions
- [ ] Point-free стиль программирования
- [ ] Монады в Rust (неформально)

## Этап 18: Макросы

### Декларативные макросы (macro_rules!)
- [ ] Синтаксис macro_rules! макросов
- [ ] Паттерны в макросах и их matching
- [ ] Метапеременные ($x:expr, $x:ident, etc.)
- [ ] Повторения в макросах ($(...)+, $(...)*,  $(...)?))
- [ ] Сепараторы в повторениях
- [ ] Рекурсивные макросы
- [ ] Hygiene и область видимости в макросах
- [ ] Экспорт макросов с #[macro_export]

### Процедурные макросы
- [ ] Derive macros (#[derive(MyMacro)])
- [ ] Attribute macros (#[my_attribute])
- [ ] Function-like macros (my_macro!(...))
- [ ] TokenStream и syn/quote crates
- [ ] Parsing Rust code в процедурных макросах
- [ ] Генерация кода с quote!

### Атрибуты и аннотации
- [ ] Встроенные атрибуты Rust
- [ ] #[derive] атрибут и его расширения
- [ ] Условная компиляция с #[cfg] и cfg!
- [ ] #[allow], #[warn], #[deny], #[forbid]
- [ ] #[inline] и оптимизации
- [ ] #[repr] для контроля memory layout
- [ ] #[test] и unit testing
- [ ] Документационные атрибуты #[doc]

## Этап 19: Небезопасный Rust

### Unsafe блоки и операции
- [ ] Пять супер-способностей unsafe Rust
- [ ] Когда использовать unsafe (и когда не использовать)
- [ ] Разыменование сырых указателей
- [ ] Вызов unsafe функций и методов
- [ ] Доступ к mutable static переменным
- [ ] Реализация unsafe трейтов
- [ ] Создание union типов

### Сырые указатели и memory management
- [ ] *const T и *mut T типы
- [ ] Создание сырых указателей из ссылок
- [ ] ptr::null() и ptr::null_mut()
- [ ] Арифметика указателей (offset, add, sub)
- [ ] Разыменование с осторожностью
- [ ] Проверка на null указатели
- [ ] Взаимодействие с C API

### FFI (Foreign Function Interface)
- [ ] extern блоки для C функций
- [ ] Calling conventions
- [ ] Типы совместимые с C (repr(C))
- [ ] Строки и C strings (CStr, CString)
- [ ] Передача колбэков в C
- [ ] bindgen для автогенерации биндингов
- [ ] Memory safety в FFI контексте

### Продвинутые unsafe паттерны
- [ ] Создание собственных коллекций
- [ ] Phantom types для zero-cost abstractions
- [ ] Variance и lifetime polymorphism
- [ ] Drop checking и dropck_eyepatch
- [ ] Miri для тестирования unsafe кода

## Этап 20: Продвинутые типы и система типов

### Type aliases и newtype pattern
- [ ] Создание псевдонимов типов с type
- [ ] Обобщенные псевдонимы типов (GATs preview)
- [ ] Never type (!) и diverging functions
- [ ] Newtype pattern для type safety
- [ ] Zero-cost wrappers
- [ ] Phantom data для marker types

### Продвинутые функции и замыкания
- [ ] Указатели на функции fn
- [ ] Различия между Fn, FnMut, FnOnce
- [ ] Возвращение замыканий из функций (Box<dyn Fn>)
- [ ] impl Trait в возвращаемых типах
- [ ] Дивергентные функции (never return)
- [ ] const fn и compile-time computation

### Продвинутые трейты и обобщенность
- [ ] Ассоциированные типы vs обобщенные параметры
- [ ] Default generic type parameters
- [ ] Fully qualified syntax (UFCS)
- [ ] Supertraits и trait bounds
- [ ] Higher-ranked trait bounds (for<'a>)
- [ ] Trait objects и dynamic dispatch
- [ ] Object safety правила

### Система времени жизни (lifetimes)
- [ ] Продвинутые lifetime аннотации
- [ ] Multiple lifetime parameters
- [ ] Lifetime elision rules подробно
- [ ] Static lifetime и его использование
- [ ] Lifetime bounds в обобщенных типах
- [ ] Higher-ranked lifetimes
- [ ] Variance в lifetimes

## Этап 21: Метапрограммирование и compile-time вычисления

### Const generics и типы времени компиляции
- [ ] Const generic parameters
- [ ] Const expressions и их ограничения
- [ ] Const fn функции продвинуто
- [ ] Compile-time вычисления и оптимизации
- [ ] const assertions для compile-time проверок
- [ ] Type-level программирование basics

### Build scripts и code generation
- [ ] build.rs скрипты и их возможности
- [ ] Генерация кода во время сборки
- [ ] Интеграция с внешними инструментами
- [ ] Environment variables в build time
- [ ] Conditional compilation продвинуто
- [ ] Proc-macro workshops setup

### Reflection и introspection
- [ ] std::any::TypeId для type identification
- [ ] downcasting с Any trait
- [ ] std::mem модуль для memory introspection
- [ ] size_of, align_of и layout информация
- [ ] std::intrinsics (unstable) обзор

## Этап 22: Производительность и оптимизации

### Профилирование и бенчмаркинг
- [ ] Criterion.rs для микробенчмаркинга
- [ ] Flamegraph для визуализации производительности
- [ ] perf и другие системные профайлеры
- [ ] cargo bench и его настройка
- [ ] Black box оптимизации для точных измерений
- [ ] Statistical significance в бенчмарках

### Оптимизации компилятора
- [ ] LLVM оптимизации и Rust
- [ ] Инлайнинг функций (#[inline])
- [ ] Link-time optimization (LTO)
- [ ] Profile-guided optimization (PGO)
- [ ] Target-specific optimizations
- [ ] cargo bloat для анализа размера бинарников

### Memory layout и cache efficiency
- [ ] Структура данных и cache locality
- [ ] Padding и alignment в структурах
- [ ] #[repr] атрибуты для контроля layout
- [ ] SIMD инструкции basics
- [ ] Branch prediction и его влияние
- [ ] False sharing в многопоточном коде

### Zero-cost abstractions анализ
- [ ] Что означает "zero-cost"
- [ ] Анализ сгенерированного assembly
- [ ] Iterators vs loops производительность
- [ ] Smart pointers overhead
- [ ] Trait objects vs static dispatch
- [ ] Monomorphization costs

## Этап 23: Экосистема и инструментарий

### Cargo ecosystem продвинуто
- [ ] Custom cargo commands
- [ ] cargo-edit, cargo-watch, cargo-expand
- [ ] Cargo workspaces для больших проектов
- [ ] Publishing на crates.io
- [ ] Semantic versioning стратегии
- [ ] Feature flags и conditional compilation
- [ ] Private registries и альтернативы

### Testing продвинуто
- [ ] Property-based testing с quickcheck
- [ ] Integration tests организация
- [ ] Mock объекты и test doubles
- [ ] Async testing patterns
- [ ] Performance regression testing
- [ ] Fuzz testing с cargo-fuzz
- [ ] Test coverage с tarpaulin

### Documentation и API design
- [ ] rustdoc продвинутые возможности
- [ ] Intra-doc links
- [ ] Doc tests и их best practices
- [ ] mdBook для документации проектов
- [ ] API design guidelines
- [ ] Deprecation strategies
- [ ] Semantic versioning для API changes

### Линтинг и code quality
- [ ] Clippy продвинутые линты
- [ ] Настройка clippy для команды
- [ ] rustfmt и code style consistency
- [ ] Custom lints написание
- [ ] Pre-commit hooks setup
- [ ] CI/CD pipelines для Rust проектов

## Этап 24: Специализированные домены

### Web development
- [ ] Actix-web фреймворк basics
- [ ] Warp и async web services
- [ ] Serde для JSON/XML сериализации
- [ ] Database интеграция (Diesel, SQLx)
- [ ] WebAssembly compilation
- [ ] Frontend с Yew или similar

### Systems programming
- [ ] OS development basics в Rust
- [ ] Embedded programming с no_std
- [ ] Real-time systems constraints
- [ ] Device drivers написание
- [ ] Custom allocators реализация
- [ ] Interrupt handling

### Network programming
- [ ] TCP/UDP сокеты в async контексте
- [ ] Protocol implementation
- [ ] Serialization protocols (bincode, protobuf)
- [ ] Async I/O patterns
- [ ] Load balancing и connection pooling
- [ ] Security considerations

### Data science и computation
- [ ] ndarray для numerical computing
- [ ] Polars для data processing
- [ ] Rayon для data parallelism
- [ ] GPU computing с wgpu
- [ ] Machine learning libraries обзор
- [ ] Scientific computing patterns

## Этап 25: Архитектура и design patterns

### Architectural patterns
- [ ] Hexagonal architecture в Rust
- [ ] Domain-driven design применение
- [ ] Event sourcing patterns
- [ ] CQRS implementation
- [ ] Microservices архитектура
- [ ] Plugin systems design

### Design patterns адаптация
- [ ] Builder pattern в идиоматическом Rust
- [ ] Factory patterns и их варианты
- [ ] Observer pattern с channels
- [ ] Strategy pattern с trait objects
- [ ] State machines implementation
- [ ] Visitor pattern alternatives

### Error handling архитектура
- [ ] Error types hierarchy design
- [ ] anyhow vs thiserror выбор
- [ ] Error context и debugging информация
- [ ] Recoverable vs non-recoverable errors
- [ ] Error logging и monitoring
- [ ] Graceful degradation patterns

### Concurrency architectures
- [ ] Actor model с actix
- [ ] CSP (Communicating Sequential Processes)
- [ ] Producer-consumer patterns
- [ ] Work stealing algorithms
- [ ] Lock-free data structures design
- [ ] Back-pressure handling

## Этап 26: Продвинутые темы и исследования

### Compiler internals понимание
- [ ] Rust compilation process
- [ ] Borrow checker алгоритмы
- [ ] MIR (Mid-level IR) понимание
- [ ] Trait resolution process
- [ ] Monomorphization details
- [ ] Incremental compilation

### Experimental features
- [ ] Nightly Rust features tracking
- [ ] Generic associated types (GATs)
- [ ] Async closures development
- [ ] const generics расширения
- [ ] Specialization (когда стабилизируется)
- [ ] Try blocks и ? оператор расширения

### Research areas
- [ ] Linear types и affine types
- [ ] Effect systems в Rust
- [ ] Gradual typing research
- [ ] Rust formal verification
- [ ] Concurrency models исследования
- [ ] Memory safety доказательства

### Contributing to Rust
- [ ] Rust RFC process понимание
- [ ] Contributing to rustc
- [ ] Ecosystem crates development
- [ ] Community participation
- [ ] Mentoring и knowledge sharing
- [ ] Speaking at conferences