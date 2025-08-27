#![no_std]

use soroban_sdk::{contract, contractimpl, contracterror, contracttype, Env, String, Symbol, Vec, symbol_short};

/// Enum que representa el estado de un libro
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BookStatus {
    Available,
    Borrowed,
    Reserved,
}

/// Struct que representa un libro en la biblioteca
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
    pub status: BookStatus,
}

/// Enum para errores personalizados del contrato
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum LibraryError {
    BookNotFound = 1,
    BookNotAvailable = 2,
    InvalidBookData = 3,
}

#[contract]
pub struct LibraryContract;

// Constante para la clave del próximo ID
const NEXT_ID_KEY: Symbol = symbol_short!("next_id");

#[contractimpl]
impl LibraryContract {
    /// Agrega un nuevo libro a la biblioteca
    /// Retorna Result<u32, LibraryError> - ID del libro o error
    pub fn add_book(env: Env, title: String, author: String) -> Result<u32, LibraryError> {
        // Validar que los datos no estén vacíos
        if title.len() == 0 || author.len() == 0 {
            return Err(LibraryError::InvalidBookData);
        }

        // Obtener el próximo ID disponible
        let next_id = Self::get_next_book_id(&env);

        // Crear el nuevo libro
        let new_book = Book {
            id: next_id,
            title: title.clone(),
            author: author.clone(),
            status: BookStatus::Available,
        };

        // Guardar el libro en storage usando Instance storage para datos del contrato
        env.storage().instance().set(&next_id, &new_book);

        // Actualizar el contador de IDs
        env.storage().instance().set(&NEXT_ID_KEY, &(next_id + 1));

        Ok(next_id)
    }

    /// Obtiene un libro por su ID
    /// Retorna Option<Book> - Some(book) si existe, None si no existe
    pub fn get_book(env: Env, book_id: u32) -> Option<Book> {
        env.storage().instance().get(&book_id)
    }

    /// Cambia el estado de un libro
    /// Retorna Result<(), LibraryError>
    pub fn change_book_status(
        env: Env,
        book_id: u32,
        new_status: BookStatus,
    ) -> Result<(), LibraryError> {
        // Obtener el libro actual
        let mut book: Book = env
            .storage()
            .instance()
            .get(&book_id)
            .ok_or(LibraryError::BookNotFound)?;

        // Actualizar el estado
        book.status = new_status;

        // Guardar el libro actualizado
        env.storage().instance().set(&book_id, &book);

        Ok(())
    }

    /// Intenta tomar prestado un libro
    /// Retorna Result<(), LibraryError>
    pub fn borrow_book(env: Env, book_id: u32) -> Result<(), LibraryError> {
        // Obtener el libro
        let book: Book = env
            .storage()
            .instance()
            .get(&book_id)
            .ok_or(LibraryError::BookNotFound)?;

        // Verificar si está disponible
        match book.status {
            BookStatus::Available => {
                // Cambiar estado a prestado
                Self::change_book_status(env, book_id, BookStatus::Borrowed)
            }
            _ => Err(LibraryError::BookNotAvailable),
        }
    }

    /// Devuelve un libro prestado (cambia estado a Available)
    /// Retorna Result<(), LibraryError>
    pub fn return_book(env: Env, book_id: u32) -> Result<(), LibraryError> {
        // Obtener el libro
        let book: Book = env
            .storage()
            .instance()
            .get(&book_id)
            .ok_or(LibraryError::BookNotFound)?;

        // Verificar si está prestado
        match book.status {
            BookStatus::Borrowed => {
                // Cambiar estado a disponible
                Self::change_book_status(env, book_id, BookStatus::Available)
            }
            _ => Err(LibraryError::BookNotAvailable),
        }
    }

    /// Función helper para obtener el próximo ID disponible
    fn get_next_book_id(env: &Env) -> u32 {
        env.storage().instance().get(&NEXT_ID_KEY).unwrap_or(1)
    }

    /// Obtiene todos los libros disponibles
    /// Retorna Vec<Book>
    pub fn get_available_books(env: Env) -> Vec<Book> {
        let mut available_books = Vec::new(&env);
        let next_id = Self::get_next_book_id(&env);

        // Iterar sobre todos los libros existentes
        for id in 1..next_id {
            if let Some(book) = Self::get_book(env.clone(), id) {
                if book.status == BookStatus::Available {
                    available_books.push_back(book);
                }
            }
        }

        available_books
    }

    /// Obtiene todos los libros en la biblioteca
    /// Retorna Vec<Book>
    pub fn get_all_books(env: Env) -> Vec<Book> {
        let mut all_books = Vec::new(&env);
        let next_id = Self::get_next_book_id(&env);

        // Iterar sobre todos los libros existentes
        for id in 1..next_id {
            if let Some(book) = Self::get_book(env.clone(), id) {
                all_books.push_back(book);
            }
        }

        all_books
    }

    /// Reserva un libro disponible
    /// Retorna Result<(), LibraryError>
    pub fn reserve_book(env: Env, book_id: u32) -> Result<(), LibraryError> {
        // Obtener el libro
        let book: Book = env
            .storage()
            .instance()
            .get(&book_id)
            .ok_or(LibraryError::BookNotFound)?;

        // Verificar si está disponible
        match book.status {
            BookStatus::Available => {
                // Cambiar estado a reservado
                Self::change_book_status(env, book_id, BookStatus::Reserved)
            }
            _ => Err(LibraryError::BookNotAvailable),
        }
    }
}

mod test;