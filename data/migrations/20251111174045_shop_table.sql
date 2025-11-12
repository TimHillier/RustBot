CREATE TABLE shop_items (
                      id INTEGER PRIMARY KEY AUTOINCREMENT,
                      item_name TEXT NOT NULL,
                      short_name TEXT NOT NULL,
                      price INTEGER NOT NULL,
                      description TEXT NOT NULL,
                      current_amount INTEGER NOT NULL,
                      total_amount INTEGER NOT NULL
);

