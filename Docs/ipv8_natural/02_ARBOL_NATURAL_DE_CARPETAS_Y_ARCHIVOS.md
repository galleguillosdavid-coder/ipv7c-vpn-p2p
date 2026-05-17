# Arbol natural de carpetas y archivos de ipv8

## Principio general

Este documento no define sintaxis de codigo.
Define significado.
Cada carpeta y cada archivo son descritos como si fueran habitaciones de una misma ciudad.

## Raiz del proyecto

En la raiz de `ipv8` solo deberian vivir los archivos de identidad del repositorio y las carpetas maestras del sistema.
La raiz no debe convertirse en un deposito de experimentos, binarios sueltos, notas temporales o restos de sesiones previas.

## Carpeta `core`

Esta sera la casa del nucleo real.
Todo lo que viva aqui debe ser parte del corazon tecnico estable de `ipv8`.
Si algo es opcional, experimental, altamente visual o dependiente del entorno de usuario, probablemente no pertenece aqui.

### Archivo `core/identidad`

Este archivo o grupo de archivos definira que es un nodo, como se nombra, como se presenta, como se firma y como conserva su continuidad entre reinicios.

### Archivo `core/confianza`

Este archivo describira reputacion, niveles de confianza, herencia de permisos y trazabilidad social o institucional del nodo.

### Archivo `core/sesiones`

Este archivo describira como nace, vive, rota y muere una sesion entre nodos.

### Archivo `core/rutas`

Este archivo describira como `ipv8` decide por donde pasa el trafico, como cambia de ruta, como degrada con elegancia y como registra por que eligio un camino en vez de otro.

### Archivo `core/fragmentacion`

Este archivo explicara como se divide la informacion, como se vuelve a unir y que reglas protegen al sistema de fragmentaciones absurdas o costosas.

### Archivo `core/telemetria`

Este archivo explicara que observa el sistema desde dentro, con que granularidad y con que proposito humano.

## Carpeta `control_plane`

Aqui vivira la politica ejecutable del sistema.
No el trafico.
No la visualizacion.
No la instalacion.
Aqui vive la decision.

### Archivo `control_plane/grupos`

Describe grupos, subgrupos, ambitos, membresias y herencias.

### Archivo `control_plane/dispositivos`

Describe tipos de dispositivo, perfiles, limitaciones y capacidades esperables.

### Archivo `control_plane/politicas`

Describe reglas de acceso, limites de accion, prioridades y conflictos de politica.

### Archivo `control_plane/evaluacion`

Describe como se produce una decision final cuando un nodo o una operacion solicita permiso.

## Carpeta `discovery`

Aqui vivira la logica que encuentra, registra y reevalua a los peers.

### Archivo `discovery/local`

Explica como se realiza descubrimiento en entornos cercanos, por ejemplo redes locales, segmentos conocidos o dominios controlados.

### Archivo `discovery/remoto`

Explica como se realiza descubrimiento fuera del entorno local, como se aprovechan semillas, como se revalida informacion y como se evita depender de un solo punto de anclaje.

### Archivo `discovery/catalogo`

Explica como se indexan capacidades, funciones y roles de los nodos sin obligar a que todos participen igual.

### Archivo `discovery/topologia`

Explica como se construye una imagen util y resumida de la red, apta para observabilidad y decisiones de control.

## Carpeta `observability`

Aqui vivira todo lo necesario para mirar la verdad del sistema.

### Archivo `observability/logs`

Describe eventos narrables, trazas resumidas y registro cronologico interpretable.

### Archivo `observability/snapshots`

Describe fotografias del estado del nodo y de la red en momentos concretos.

### Archivo `observability/metrics`

Describe contadores, tasas, latencias, pesos y señales que importan a operacion.

### Archivo `observability/audit`

Describe como se registran decisiones importantes, reparaciones, cambios de politica y hechos de seguridad.

## Carpeta `automation`

Aqui viviran los asistentes, runbooks y mecanismos para delegar sin perder control.

### Archivo `automation/runbooks`

Describe secuencias aprobadas para instalar, reparar, validar, migrar y auditar.

### Archivo `automation/agents`

Describe tipos de agentes permitidos, sus limites y las tareas que pueden proponer o ejecutar en el futuro.

### Archivo `automation/translation`

Describe como se transforma un estado tecnico en un informe humano, corto, claro y trazable.

## Carpeta `desktop`

Aqui vivira la experiencia local de escritorio.
No la logica total del sistema.
No la politica central.
No el kernel.
La experiencia.

### Archivo `desktop/panel`

Describe las vistas principales del panel de control.

### Archivo `desktop/estado`

Describe como el usuario comprende salud, conectividad, reputacion, sesiones y alertas.

### Archivo `desktop/acciones`

Describe que acciones humanas existen, cuales son seguras, cuales piden confirmacion y cuales nunca deberian estar a un clic accidental.

## Carpeta `mobile`

Aqui vivira la experiencia movil, pensada como adaptacion de `ipv8`, no como copia exacta del escritorio.

## Carpeta `edge`

Aqui viviran perfiles pequenos, gateways, routers, nodos de campo y despliegues de menor huella.

## Carpeta `lab`

Aqui vivira lo experimental.
Todo lo que entre a `lab` debe declararse como no canonico hasta que exista evidencia suficiente para ascenderlo a otra capa.

## Carpeta `docs`

Aqui vivira la explicacion del sistema para humanos y para otras IAs.
No deberia ser un museo caotico.
Deberia ser una biblioteca con indices claros.

## Carpeta `migrations`

Aqui viviran las guias de paso desde `ipv7c`, desde `antiguos` y desde cualquier implementacion intermedia.

## Carpeta `tests`

Aqui viviran los escenarios de validacion descritos primero en lenguaje humano y despues transformados en automatizacion por partes.

## Carpeta `benchmarks`

Aqui viviran las comparaciones de rendimiento, costo y estabilidad.
No para presumir.
Para decidir con evidencia.

## Regla final del arbol

Si aparece una carpeta nueva, debe poder responder tres preguntas.
Que responsabilidad tiene.
Que responsabilidad no tiene.
Con que otras carpetas puede hablar y bajo que contrato.

Si una carpeta no puede responder esas preguntas en lenguaje natural, no deberia entrar todavia al arbol oficial de `ipv8`.
