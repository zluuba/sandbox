# Practical API Design: Confessions of a Java Framework Architect
Practicing patterns with the book 'Practical API Design: Confessions of a Java Framework Architect'.  


## Chapter 7
- anagram_game (dependency injection)
- lookup (NetBeans Lookup/modularity)

### Lookup (NetBeans)
NetBeans Lookup is a core component of the NetBeans Platform, providing a flexible and powerful mechanism for service discovery and loose coupling between modules.  

#### Keys
Service Provider: object that provides a specific service or functionality.  
Lookup: container that holds service providers and allows clients to query for them.  
Global Lookup: platform-wide lookup that contains services available to all modules.  
Context Lookup: lookup associated with a specific context, like a node in a tree or a window.  
Lookup Listener: for dynamic service discovery.  

#### How It Works
- service providers register themselves with a Lookup.
- clients query the Lookup for services they need.
- the Lookup returns the requested service if available.

#### Simple Example
```java
// Get the global lookup
Lookup globalLookup = Lookup.getDefault();

// Query for a service
SomeService service = globalLookup.lookup(SomeService.class);

if (service != null) {
    // Use the service
    service.doSomething();
} else {
    // Handle case where service is not available
}
```

#### Note
While NetBeans Lookup is inspired by the general lookup pattern, it extends the concept significantly to address specific needs in modular Java application development.
The **general principles of the lookup pattern** - efficient retrieval of data or services based on a key - are indeed common across many languages and frameworks.  
For example:
- in Python, you might use dictionaries for lookup
- in Rust, you might use HashMap 
- in JavaScript, objects or Maps are often used for similar purposes
  
