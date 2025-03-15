CREATE TABLE "bot" (
	"id"	TEXT NOT NULL UNIQUE,
	"name"	TEXT NOT NULL,
	"about"	TEXT,
	"avatar"	TEXT,
	"banner"	TEXT,
	"developer"	TEXT NOT NULL,
    "visibility" INTEGER NOT NULL
);

CREATE TABLE "client" (
	"id"	INTEGER NOT NULL UNIQUE,
	"name"	TEXT NOT NULL,
	"developer"	TEXT NOT NULL,
	"platform"	INTEGER NOT NULL,
	"description"	TEXT,
	"icon"	TEXT,
	"source"	TEXT NOT NULL UNIQUE,
    "visibility" INTEGER NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);

CREATE TABLE "plugin" (
	"name"	TEXT NOT NULL UNIQUE,
	"description"	TEXT,
	"developer"	TEXT NOT NULL,
	"source"	TEXT NOT NULL UNIQUE,
    "visibility" INTEGER NOT NULL,
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
	"invite"	TEXT NOT NULL UNIQUE,
    "visibility" INTEGER NOT NULL,
	PRIMARY KEY("id")
);

CREATE TABLE "theme" (
	"name"	TEXT NOT NULL UNIQUE,
	"description"	TEXT,
	"author"	TEXT NOT NULL,
	"data"	BLOB,
	"platform"	INTEGER NOT NULL,
    "visibility" INTEGER NOT NULL,
	PRIMARY KEY("name")
);

CREATE TABLE "report" (
    "id"	INTEGER NOT NULL UNIQUE,
    "reporter_id"	TEXT NOT NULL,
    "reported_id"	TEXT NOT NULL,
    "reported_type" INTEGER NOT NULL,
    "reason"	TEXT NOT NULL,
    "status"	INTEGER NOT NULL,
    PRIMARY KEY("id" AUTOINCREMENT)
)