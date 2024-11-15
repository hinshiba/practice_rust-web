-- Add up migration script here
CREATE TABLE equips (
    id INTEGER PRIMARY KEY,
    applicant_name TEXT NOT NULL,
    disposaler_name TEXT NOT NULL,
    equipment_type TEXT NOT NULL,
    equipment_name TEXT NOT NULL,
    purchase_url TEXT NOT NULL,
    cost INTEGER NOT NULL,
    situation TEXT CHECK(situation IN ('pending', 'approved','purchased', 'equipment')) NOT NULL DEFAULT 'pending',
    approval_date TEXT,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP
);