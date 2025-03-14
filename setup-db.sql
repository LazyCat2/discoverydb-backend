CREATE TABLE "bot" (
	"id"	TEXT NOT NULL UNIQUE,
	"name"	TEXT NOT NULL,
	"about"	TEXT,
	"avatar"	TEXT,
	"banner"	TEXT,
	"developer"	TEXT NOT NULL
);

CREATE TABLE "client" (
	"id"	INTEGER NOT NULL UNIQUE,
	"name"	TEXT NOT NULL,
	"developer"	TEXT NOT NULL,
	"platform"	INTEGER NOT NULL,
	"description"	TEXT,
	"icon"	TEXT,
	"source"	TEXT NOT NULL UNIQUE,
	PRIMARY KEY("id" AUTOINCREMENT)
);

CREATE TABLE "plugin" (
	"name"	TEXT NOT NULL UNIQUE,
	"description"	TEXT,
	"developer"	TEXT NOT NULL,
	"source"	TEXT NOT NULL UNIQUE,
	PRIMARY KEY("name")
);

CREATE TABLE "server" (
	"id"	TEXT NOT NULL UNIQUE,
	"name"	TEXT NOT NULL,
	"description"	TEXT,
	"icon"	TEXT,
	"banner"	TEXT,
	"owner"	TEXT NOT NULL,
	"members"	INTEGER NOT NULL,
	"invite"	INTEGER NOT NULL UNIQUE,
	PRIMARY KEY("id")
);

CREATE TABLE "theme" (
	"name"	TEXT NOT NULL UNIQUE,
	"description"	TEXT,
	"author"	TEXT NOT NULL,
	"data"	BLOB,
	"platform"	INTEGER NOT NULL,
	PRIMARY KEY("name")
);
