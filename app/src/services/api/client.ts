import axios from "axios";

export const CLIENT = axios.create({
	baseURL: process.env.API_URL,
});
