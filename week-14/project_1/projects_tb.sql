--
-- PostgreSQL database dump
--

\restrict tcbG6IrB57Y43VEWWF2ceHnTKdcwjouzcgjRFgp9tY5WCTERVJYe40kPViMEboE

-- Dumped from database version 18.6
-- Dumped by pg_dump version 18.6

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: projects; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.projects (
    p_number integer NOT NULL,
    p_name character(50) NOT NULL,
    p_duration integer NOT NULL,
    project_manager_id integer NOT NULL
);


ALTER TABLE public.projects OWNER TO postgres;

--
-- Data for Name: projects; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.projects (p_number, p_name, p_duration, project_manager_id) FROM stdin;
44	A                                                 	9	102
11	B                                                 	14	97
44	C                                                 	16	120
33	D                                                 	25	108
22	E                                                 	9	107
\.


--
-- Name: projects projects_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.projects
    ADD CONSTRAINT projects_pkey PRIMARY KEY (project_manager_id);


--
-- PostgreSQL database dump complete
--

\unrestrict tcbG6IrB57Y43VEWWF2ceHnTKdcwjouzcgjRFgp9tY5WCTERVJYe40kPViMEboE

