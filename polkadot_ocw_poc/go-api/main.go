package main

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"sync"
)

// Item struct represents a simple data structure
type Item struct {
	ID   int    `json:"id"`
	Name string `json:"name"`
}

var (
	items      = make(map[int]Item)
	itemsMutex sync.RWMutex
)

func main() {
	// Define your API endpoints
	http.HandleFunc("/spawn", spawNode)

	// Start the HTTP server
	port := 8080
	fmt.Printf("Server listening on :%d...\n", port)
	err := http.ListenAndServe(fmt.Sprintf(":%d", port), nil)
	if err != nil {
		fmt.Println(err)
	}
}

func spawNode(w http.ResponseWriter, r *http.Request) {
	fmt.Println("Helloo")
	// Decode the JSON request body into an Item
	newItem := &Node{}
	re, err := io.ReadAll(r.Body)
	if err != nil {
		http.Error(w, "Invalid JSON format", http.StatusBadRequest)
		return
	}

	fmt.Println(string(re))
	err = json.Unmarshal(re, newItem)
	if err != nil {
		fmt.Println(err)
		http.Error(w, "Invalid JSON format", http.StatusBadRequest)
		return
	}

	fmt.Printf("decode %v", newItem)

	itemsMutex.Lock()
	defer itemsMutex.Unlock()

	resullt := KurtosisCall(*newItem)

	res, err := json.Marshal(resullt)
	// Respond with the newly added item
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(string(res))
}
