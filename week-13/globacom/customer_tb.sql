--
-- PostgreSQL database dump
--

-- Dumped from database version 17.2
-- Dumped by pg_dump version 17.2

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
-- Name: customer; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.customer (
    c_id integer NOT NULL,
    c_name text NOT NULL,
    c_age character varying(3) NOT NULL,
    c_email character(25) NOT NULL,
    c_mobile character varying(12) NOT NULL,
    eid character varying(4) NOT NULL,
    data_id character varying(3) NOT NULL
);


ALTER TABLE public.customer OWNER TO postgres;

--
-- Data for Name: customer; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.customer (c_id, c_name, c_age, c_email, c_mobile, eid, data_id) FROM stdin;
110	Musta Karim	35	m_karim@gmail.com        	2004433856	102	5
111	Lilian Jaiya	43	I_jaiye@gmail.com        	2004433556	100	3
112	Arthur Musa	50	a_musa@gmail.com         	2004433555	107	10
113	Philip Akonjo	41	p_akonjo@gmail.com       	2004432555	100	2
114	Marylene Mapa	33	m_mapa@gmail.com         	2005432555	120	5
115	Oghenero Agor	50	o_agor@gmail.com         	2015432555	117	11
116	Adams Bree	33	a_bree@gmail.com         	2025432555	102	1
117	Okafor Mathais	45	o_mathais@gmail.com      	2065432555	120	10
118	Samson Adeleke	65	s_adeleke@gmail.com      	2045432555	117	11
119	Lawal Tamire	35	i_tamire@gmail.com       	2045632555	107	5
120	James Job	44	j_job@gmail.com          	2045662555	100	8
121	Matthew Jakande	21	m_jakande@gmail.com      	2047662555	120	2
122	Jimila Adegboye	20	j_adegboye@gmail.com     	2047662565	107	5
\.


--
-- Name: customer customer_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customer
    ADD CONSTRAINT customer_pkey PRIMARY KEY (c_id);


--
-- PostgreSQL database dump complete
--

