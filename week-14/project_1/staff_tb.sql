--
-- PostgreSQL database dump
--

\restrict azm0vqhb7d5FTXubafZlpwDzl8v5Wak0QCk6semdgm2ZKFCef9tZU2e1Mj3EmZL

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
-- Name: staff; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.staff (
    staff_id integer CONSTRAINT employees_e_id_not_null NOT NULL,
    staff_name character(50) CONSTRAINT employees_e_name_not_null NOT NULL,
    d_number integer CONSTRAINT employees_d_number_not_null NOT NULL,
    staff_sal double precision,
    age integer CONSTRAINT employees_age_not_null NOT NULL,
    mobile character varying(15) NOT NULL
);


ALTER TABLE public.staff OWNER TO postgres;

--
-- Data for Name: staff; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.staff (staff_id, staff_name, d_number, staff_sal, age, mobile) FROM stdin;
100	Mustaph Ali                                       	3	150000	31	801234568
107	Alokwe Martin                                     	2	250000	35	8012345677
97	Dankade Aminat                                    	7	350000	25	8012345676
108	Josiah Joshua                                     	8	260000	45	8012345675
102	Makinde Mary                                      	6	270000	42	8012345674
120	Adeleke Jane                                      	3	280000	21	8012345673
122	Osahan Mark                                       	6	290000	24	8012345672
117	Suleman Ajayi                                     	5	300000	42	8012345671
104	Kuti Lawal                                        	1	310000	36	8012345670
\.


--
-- Name: staff employees_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT employees_pkey PRIMARY KEY (staff_id);


--
-- PostgreSQL database dump complete
--

\unrestrict azm0vqhb7d5FTXubafZlpwDzl8v5Wak0QCk6semdgm2ZKFCef9tZU2e1Mj3EmZL

