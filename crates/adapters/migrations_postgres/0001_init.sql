--
-- PostgreSQL database dump
--


-- Dumped from database version 16.10
-- Dumped by pg_dump version 16.10

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: projects; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.projects (
    repo_id text NOT NULL,
    name text,
    slug text,
    description text,
    override_description boolean DEFAULT false,
    url text,
    override_url boolean DEFAULT false,
    status text,
    logo text,
    twitter text,
    comments text,
    created_at timestamp with time zone DEFAULT now(),
    updated_at text
);


--
-- Name: repos; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.repos (
    id text NOT NULL,
    github_repo_id bigint,
    full_name text,
    stars bigint DEFAULT '0'::bigint,
    forks bigint DEFAULT '0'::bigint,
    open_issues bigint DEFAULT '0'::bigint,
    watchers bigint DEFAULT '0'::bigint,
    last_fetched_at text,
    etag text,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now()
);


--
-- Name: snapshot_deltas; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.snapshot_deltas (
    id bigint NOT NULL,
    repo_id text,
    snapshot_date date,
    prev_snapshot_date date,
    stars_delta bigint,
    forks_delta bigint,
    open_issues_delta bigint,
    watchers_delta bigint,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now()
);


--
-- Name: snapshot_deltas_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.snapshot_deltas_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: snapshot_deltas_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.snapshot_deltas_id_seq OWNED BY public.snapshot_deltas.id;


--
-- Name: snapshots; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.snapshots (
    id bigint NOT NULL,
    repo_id text,
    snapshot_date date,
    stars bigint,
    forks bigint,
    open_issues bigint,
    watchers bigint,
    fetched_at text,
    created_at timestamp with time zone DEFAULT now()
);


--
-- Name: snapshots_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.snapshots_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: snapshots_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.snapshots_id_seq OWNED BY public.snapshots.id;


--
-- Name: snapshot_deltas id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.snapshot_deltas ALTER COLUMN id SET DEFAULT nextval('public.snapshot_deltas_id_seq'::regclass);


--
-- Name: snapshots id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.snapshots ALTER COLUMN id SET DEFAULT nextval('public.snapshots_id_seq'::regclass);


--
-- Name: repos idx_16469_sqlite_autoindex_repos_1; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.repos
    ADD CONSTRAINT idx_16469_sqlite_autoindex_repos_1 PRIMARY KEY (id);


--
-- Name: snapshots idx_16481_snapshots_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.snapshots
    ADD CONSTRAINT idx_16481_snapshots_pkey PRIMARY KEY (id);


--
-- Name: snapshot_deltas idx_16489_snapshot_deltas_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.snapshot_deltas
    ADD CONSTRAINT idx_16489_snapshot_deltas_pkey PRIMARY KEY (id);


--
-- Name: projects idx_16497_sqlite_autoindex_projects_1; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.projects
    ADD CONSTRAINT idx_16497_sqlite_autoindex_projects_1 PRIMARY KEY (repo_id);


--
-- Name: idx_16469_idx_repos_full_name; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_16469_idx_repos_full_name ON public.repos USING btree (full_name);


--
-- Name: idx_16469_uq_repos_github_repo_id; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX idx_16469_uq_repos_github_repo_id ON public.repos USING btree (github_repo_id);


--
-- Name: idx_16481_idx_snapshots_repo_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_16481_idx_snapshots_repo_id ON public.snapshots USING btree (repo_id);


--
-- Name: idx_16481_idx_snapshots_snapshot_date; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_16481_idx_snapshots_snapshot_date ON public.snapshots USING btree (snapshot_date);


--
-- Name: idx_16481_uq_snapshots_repo_day; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX idx_16481_uq_snapshots_repo_day ON public.snapshots USING btree (repo_id, snapshot_date);


--
-- Name: idx_16489_idx_snapshot_deltas_repo_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_16489_idx_snapshot_deltas_repo_id ON public.snapshot_deltas USING btree (repo_id);


--
-- Name: idx_16489_idx_snapshot_deltas_snapshot_date; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_16489_idx_snapshot_deltas_snapshot_date ON public.snapshot_deltas USING btree (snapshot_date);


--
-- Name: idx_16489_uq_snapshot_deltas_repo_day; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX idx_16489_uq_snapshot_deltas_repo_day ON public.snapshot_deltas USING btree (repo_id, snapshot_date);


--
-- Name: idx_16497_sqlite_autoindex_projects_2; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX idx_16497_sqlite_autoindex_projects_2 ON public.projects USING btree (name);


--
-- Name: idx_16497_sqlite_autoindex_projects_3; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX idx_16497_sqlite_autoindex_projects_3 ON public.projects USING btree (slug);


--
-- Name: idx_repos_full_name; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_repos_full_name ON public.repos USING btree (full_name);


--
-- Name: idx_snapshot_deltas_repo_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_snapshot_deltas_repo_id ON public.snapshot_deltas USING btree (repo_id);


--
-- Name: idx_snapshot_deltas_snapshot_date; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_snapshot_deltas_snapshot_date ON public.snapshot_deltas USING btree (snapshot_date);


--
-- Name: idx_snapshots_repo_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_snapshots_repo_id ON public.snapshots USING btree (repo_id);


--
-- Name: idx_snapshots_snapshot_date; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_snapshots_snapshot_date ON public.snapshots USING btree (snapshot_date);


--
-- Name: uq_repos_github_repo_id; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX uq_repos_github_repo_id ON public.repos USING btree (github_repo_id);


--
-- Name: uq_snapshot_deltas_repo_day; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX uq_snapshot_deltas_repo_day ON public.snapshot_deltas USING btree (repo_id, snapshot_date);


--
-- Name: uq_snapshots_repo_day; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX uq_snapshots_repo_day ON public.snapshots USING btree (repo_id, snapshot_date);


--
-- Name: snapshot_deltas snapshot_deltas_repo_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.snapshot_deltas
    ADD CONSTRAINT snapshot_deltas_repo_id_fkey FOREIGN KEY (repo_id) REFERENCES public.repos(id);


--
-- Name: snapshots snapshots_repo_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.snapshots
    ADD CONSTRAINT snapshots_repo_id_fkey FOREIGN KEY (repo_id) REFERENCES public.repos(id);


--
-- PostgreSQL database dump complete
--


