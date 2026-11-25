--
-- PostgreSQL database dump
--

\restrict Qr2kRSWPZqjjMeGJHI7KrZrsKu6SSDHpb76y5LXhEYF5oB57R3i5MOYceT2xzlO

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
-- Name: customer_table; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.customer_table (
    c_id integer NOT NULL,
    c_name character varying(60) NOT NULL,
    c_age smallint NOT NULL,
    c_email character varying(70) NOT NULL,
    c_mobile character varying(15) NOT NULL,
    e_id integer NOT NULL,
    data_id integer NOT NULL
);


ALTER TABLE public.customer_table OWNER TO postgres;

--
-- Name: dataplan; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.dataplan (
    data_id integer NOT NULL,
    data_size character varying(10) NOT NULL,
    data_duration smallint NOT NULL,
    data_price integer NOT NULL
);


ALTER TABLE public.dataplan OWNER TO postgres;

--
-- Name: departments; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.departments (
    dept_manager_id integer NOT NULL,
    d_number integer NOT NULL,
    d_name character(40) NOT NULL,
    d_location character(40) NOT NULL,
    p_number integer NOT NULL
);


ALTER TABLE public.departments OWNER TO postgres;

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
-- Data for Name: customer_table; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.customer_table (c_id, c_name, c_age, c_email, c_mobile, e_id, data_id) FROM stdin;
110	Musta Karim	35	m_karim@gmail.com	8055089112	102	5
111	Lilian Jaiya	43	I_jaiyem@gmail.com	8055185341	100	3
112	Arthur Musa	50	a_musa@gmail.com	7055282813	107	10
113	Philip Akonjo	41	p_akonjom@gmail.com	9052356772	100	2
114	Marylene Mapa 	33	m_mapa@gmail.com	8053333551	120	5
115	Oghenero Agor	50	o_agor@gmail.com	7055566774	117	11
116	Adams Bree	33	a_bree@gmail.com	8056765424	102	1
117	Okafor Mathias	45	o_mathias@gmail.com	8056763367	120	10
118	Samson Adeleke	65	s_adeleke@gmail.com	7056774423	117	11
119	Lawal Tamire	35	l_tamire@gmail.com	9052111101	107	5
120	James Job	44	j_job@gmail.com	8059693919	100	8
121	Matthew Jakande	21	m_jakande@gmail.com	7051232144	120	2
122	Jimila Adegboye	20	j_adegboye@gmail.com	8054921923	107	5
\.


--
-- Data for Name: dataplan; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.dataplan (data_id, data_size, data_duration, data_price) FROM stdin;
1	350MB	2	200
2	1.8GB	14	500
3	3.9GB	30	1000
4	7.5GB	30	1500
5	9.2GB	30	2000
6	10.8GB	30	2500
7	14GB	30	3000
8	18GB	30	4000
9	24GB	30	5000
10	29.9GB	30	8000
11	50GB	30	10000
\.


--
-- Data for Name: departments; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.departments (dept_manager_id, d_number, d_name, d_location, p_number) FROM stdin;
101	2	Administration                          	Ikeja                                   	44
100	3	Account                                 	Egbeda                                  	11
107	2	Packaging                               	Ajah                                    	44
97	7	Research                                	VI                                      	33
108	8	Account                                 	Magodo                                  	22
102	6	Operations                              	Mile 2                                  	44
120	3	Packaging                               	Ketu                                    	55
\.


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
-- Name: customer_table customer_table_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customer_table
    ADD CONSTRAINT customer_table_pkey PRIMARY KEY (c_id);


--
-- Name: dataplan dataplan_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.dataplan
    ADD CONSTRAINT dataplan_pkey PRIMARY KEY (data_id);


--
-- Name: departments departments_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.departments
    ADD CONSTRAINT departments_pkey PRIMARY KEY (dept_manager_id);


--
-- Name: staff employees_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT employees_pkey PRIMARY KEY (staff_id);


--
-- Name: projects projects_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.projects
    ADD CONSTRAINT projects_pkey PRIMARY KEY (project_manager_id);


--
-- PostgreSQL database dump complete
--

\unrestrict Qr2kRSWPZqjjMeGJHI7KrZrsKu6SSDHpb76y5LXhEYF5oB57R3i5MOYceT2xzlO

