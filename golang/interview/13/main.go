package main

import (
	"database/sql"
	"errors"
	"fmt"

	_ "github.com/mattn/go-sqlite3"
)

// Product represents a product in the inventory system
type Product struct {
	ID       int64
	Name     string
	Price    float64
	Quantity int
	Category string
}

// ProductStore manages product operations
type ProductStore struct {
	db *sql.DB
}

// NewProductStore creates a new ProductStore with the given database connection
func NewProductStore(db *sql.DB) *ProductStore {
	return &ProductStore{db: db}
}

var ErrProductNotFound = errors.New("product not found")

func InitDB(dbPath string) (*sql.DB, error) {
	db, err := sql.Open("sqlite3", dbPath)
	if err != nil {
		return nil, fmt.Errorf("open sqlite db: %w", err)
	}

	const createProductsTableQuery = `
		CREATE TABLE IF NOT EXISTS products (
			id INTEGER PRIMARY KEY,
			name TEXT NOT NULL,
			price REAL NOT NULL,
			quantity INTEGER NOT NULL,
			category TEXT NOT NULL
		);
	`

	if _, err := db.Exec(createProductsTableQuery); err != nil {
		db.Close()
		return nil, fmt.Errorf("create products table: %w", err)
	}

	return db, nil
}

func (ps *ProductStore) CreateProduct(product *Product) error {
	if product == nil {
		return errors.New("product is nil")
	}

	const insertProductQuery = `
		INSERT INTO products (name, price, quantity, category)
		VALUES (?, ?, ?, ?)
	`

	result, err := ps.db.Exec(
		insertProductQuery,
		product.Name,
		product.Price,
		product.Quantity,
		product.Category,
	)
	if err != nil {
		return fmt.Errorf("create product: %w", err)
	}

	id, err := result.LastInsertId()
	if err != nil {
		return fmt.Errorf("get inserted product id: %w", err)
	}

	product.ID = id
	return nil
}

func (ps *ProductStore) GetProduct(id int64) (*Product, error) {
	const getProductQuery = `
		SELECT id, name, price, quantity, category
		FROM products
		WHERE id = ?
	`

	product := &Product{}

	err := ps.db.QueryRow(getProductQuery, id).Scan(
		&product.ID,
		&product.Name,
		&product.Price,
		&product.Quantity,
		&product.Category,
	)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return nil, fmt.Errorf("get product %d: %w", id, ErrProductNotFound)
		}

		return nil, fmt.Errorf("get product %d: %w", id, err)
	}

	return product, nil
}

// UpdateProduct updates an existing product
func (ps *ProductStore) UpdateProduct(product *Product) error {
	if product == nil {
		return errors.New("product is nil")
	}

	const updateQuery = `
		UPDATE products
		SET name = ?, price = ?, quantity = ?, category = ?
		WHERE id = ?
	`

	result, err := ps.db.Exec(
		updateQuery,
		product.Name,
		product.Price,
		product.Quantity,
		product.Category,
		product.ID,
	)
	if err != nil {
		return fmt.Errorf("update product %d: %w", product.ID, err)
	}

	rowsAffected, err := result.RowsAffected()
	if err != nil {
		return fmt.Errorf("get updated rows for product %d: %w", product.ID, err)
	}

	if rowsAffected == 0 {
		return fmt.Errorf("update product %d: %w", product.ID, ErrProductNotFound)
	}

	return nil
}

// DeleteProduct removes a product by ID
func (ps *ProductStore) DeleteProduct(id int64) error {
	const deleteQuery = `
		DELETE FROM products
		WHERE id = ?
	`

	result, err := ps.db.Exec(deleteQuery, id)
	if err != nil {
		return fmt.Errorf("delete product %d: %w", id, err)
	}

	rowsAffected, err := result.RowsAffected()
	if err != nil {
		return fmt.Errorf("get deleted rows for product %d: %w", id, err)
	}

	if rowsAffected == 0 {
		return fmt.Errorf("delete product %d: %w", id, ErrProductNotFound)
	}

	return nil
}

func (ps *ProductStore) ListProducts(category string) ([]*Product, error) {
	query := `
		SELECT id, name, price, quantity, category
		FROM products
	`

	var args []any
	if category != "" {
		query += " WHERE category = ?"
		args = append(args, category)
	}

	rows, err := ps.db.Query(query, args...)
	if err != nil {
		return nil, fmt.Errorf("query products: %w", err)
	}
	defer rows.Close()

	products := make([]*Product, 0)

	for rows.Next() {
		product := &Product{}

		if err := rows.Scan(
			&product.ID,
			&product.Name,
			&product.Price,
			&product.Quantity,
			&product.Category,
		); err != nil {
			return nil, fmt.Errorf("scan product: %w", err)
		}

		products = append(products, product)
	}

	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("iterate products: %w", err)
	}

	return products, nil
}

func (ps *ProductStore) BatchUpdateInventory(updates map[int64]int) error {
	tx, err := ps.db.Begin()
	if err != nil {
		return fmt.Errorf("begin batch inventory update: %w", err)
	}
	defer tx.Rollback()

	const updateInventoryQuery = `
		UPDATE products
		SET quantity = ?
		WHERE id = ?
	`

	stmt, err := tx.Prepare(updateInventoryQuery)
	if err != nil {
		return fmt.Errorf("prepare inventory update: %w", err)
	}
	defer stmt.Close()

	for id, quantity := range updates {
		result, err := stmt.Exec(quantity, id)
		if err != nil {
			return fmt.Errorf("update inventory for product %d: %w", id, err)
		}

		rowsAffected, err := result.RowsAffected()
		if err != nil {
			return fmt.Errorf("get affected rows for product %d: %w", id, err)
		}

		if rowsAffected == 0 {
			return fmt.Errorf("update inventory for product %d: %w", id, ErrProductNotFound)
		}
	}

	if err := tx.Commit(); err != nil {
		return fmt.Errorf("commit batch inventory update: %w", err)
	}

	return nil
}

func main() {

}
