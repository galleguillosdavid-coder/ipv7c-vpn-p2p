# Paradigma de Comunicación de la Inteligencia Soberana (Sovereign AI)

## Principio Básico
La aplicación **nunca reporta errores pasivos, reporta acciones proactivas**.
Cualquier obstáculo técnico (conexión perdida, ruta inaccesible, puerto ocupado) no debe presentarse como un "fallo", sino como un **trigger** que inicia un proceso inteligente de búsqueda, exploración o reconfiguración.

## Reglas de Traducción Semántica

### 1. Caídas de Conexión (Backoff / Timeout / Error)
*   **Mal:** "Fallo al conectar a 192.168.1.1 tras 2 intentos." o "Ruta fallida."
*   **Bien:** "Consulté mi memoria a largo plazo y la última ruta óptima fue 192.168.1.1. Al no obtener respuesta, he activado el escáner local buscando nuevas vías y dispositivos IoT."

### 2. Desconexiones de Nodos (Peer Disconnected)
*   **Mal:** "Peer desconectado a3b4c5d6."
*   **Bien:** "El nodo a3b4c5d6 ha abandonado la malla. Reorganizando las rutas óptimas de la red y buscando alternativas."

### 3. Falta de Recursos (Puertos, Interfaces)
*   **Mal:** "Error al abrir puerto UDP" o "Ruta no agregada."
*   **Bien:** "Canal estándar no disponible. Explorando protocolos y vías alternativas de descubrimiento."

### 4. Nuevas Integraciones (Peers / Rutas / Descubrimientos)
*   **Mal:** "Ruta agregada" o "Peer activo."
*   **Bien:** "Descubriendo redes: Dispositivo/Nodo integrado a la malla inteligente exitosamente."

## Implementación en el Código
Todos los mensajes generados mediante `_event()`, `logger` o el propio chat de la IA, deben escribirse en **primera persona** (Ej: "Consulté", "Integré", "Estoy buscando") para transmitir que la red es un ente vivo, orgánico y autónomo.
