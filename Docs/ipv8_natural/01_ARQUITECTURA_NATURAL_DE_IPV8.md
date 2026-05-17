# Arquitectura natural de ipv8

## Vision general

`ipv8` sera una plataforma soberana de conectividad, coordinacion y observabilidad construida por capas claramente separadas.
No sera un unico programa gigantesco que haga todo al mismo tiempo.
No sera un manifiesto incrustado dentro de un ejecutable.
No sera un monolito donde la capa visual, la seguridad, el descubrimiento, la reparacion, la telemetria y la automatizacion compitan dentro del mismo archivo principal.

`ipv8` sera una federacion de dominios bien definidos.
Cada dominio tendra una mision concreta.
Cada dominio tendra entradas y salidas comprensibles.
Cada dominio podra evolucionar sin obligar a reescribir el resto del sistema.

## Las seis capas reales de ipv8

### Capa uno: identidad y confianza

Esta capa responde a una pregunta esencial.
Quien es cada nodo.

Aqui viviran la identidad del nodo, la reputacion basica, las huellas locales, las relaciones de confianza, la pertenencia a grupos y la trazabilidad de decisiones soberanas.

Esta capa no transporta paquetes por si sola.
No dibuja dashboards.
No ejecuta agentes.
Su trabajo es declarar la identidad, la legitimidad y el contexto social o politico de cada participante de la red.

### Capa dos: plano de datos

Esta capa responde a otra pregunta.
Como se mueve el trafico.

Aqui viviran las sesiones, las rutas, la fragmentacion, la priorizacion, la recuperacion de perdida, la adaptacion de MTU, la multiplexacion y la forma concreta en que el sistema transporta contenido entre nodos.

Esta capa no decide politicas humanas complejas.
No define experiencia de usuario.
No deberia mezclar su trabajo con scripts de instalacion o con asistentes autonomos.

### Capa tres: plano de control

Esta capa responde a la pregunta.
Quien puede hacer que dentro del sistema.

Aqui viviran grupos, dispositivos, capacidades, politicas, reglas de evaluacion, permisos de mision, limites de rol, perfiles de despliegue y decisiones de admision o rechazo.

Esta capa no debe confundirse con el plano de datos.
El plano de datos mueve.
El plano de control decide.

### Capa cuatro: descubrimiento y topologia

Esta capa responde a la pregunta.
Como se encuentran los nodos y como entienden la forma de la red.

Aqui viviran semillas, directorios distribuidos, observacion de peers, descubrimiento local, descubrimiento remoto, catalogos de capacidades, DHT liviana, mecanismos de bootstrap y lectura de salud topologica.

Esta capa no deberia secuestrar la experiencia de usuario con detalles excesivos.
Su funcion es dar soporte a la red, no convertirse en el unico centro narrativo del producto.

### Capa cinco: observabilidad y memoria operativa

Esta capa responde a la pregunta.
Como sabemos que esta pasando.

Aqui viviran logs, eventos, contadores, snapshots, auditorias, estados de salud, vistas para dashboards, bitacoras para remediacion y memoria operativa suficiente para explicar decisiones importantes del nodo.

Esta capa no existe solo para mirar bonito.
Existe para decir la verdad del sistema de forma interpretable.

### Capa seis: automatizacion y gobierno asistido

Esta capa responde a la pregunta.
Que puede delegarse sin romper soberania.

Aqui viviran runbooks, agentes permitidos, tareas guiadas, propuestas de reparacion, sugerencias, traducciones de estado tecnico a lenguaje humano y puentes documentales para otras IAs.

Esta capa nunca debe tomar el control total del sistema sin restricciones.
Su papel es asistir, no reemplazar la gobernanza del proyecto.

## Los cuatro perfiles de despliegue

### Perfil minimo

Para nodos simples.
Para pruebas locales.
Para entornos donde se necesita una red pequena, comprensible y sin demasiadas piezas activas.

### Perfil edge

Para laptops, escritorios, estaciones de campo y nodos de paso.
Este perfil prioriza adaptabilidad, observabilidad razonable y consumo controlado.

### Perfil gateway

Para routers, servidores de borde y nodos que abren camino a otros.
Este perfil enfatiza politicas, ruteo, descubrimiento y estabilidad sostenida.

### Perfil laboratorio soberano

Para investigacion, simulacion controlada, auditoria, comparacion de estrategias y experimentos.
Este perfil debe quedar aislado de cualquier modo que se considere base productiva.

## Las tres reglas de oro de ipv8

Primera regla.
Nada importante vive solo en una idea implicita.
Todo modulo importante debe poder explicarse, versionarse y auditarse.

Segunda regla.
La complejidad debe estar segmentada por dominio y no comprimida dentro de un archivo heroico.

Tercera regla.
La migracion desde `ipv7c` y desde `antiguos` no debe copiar el pasado entero.
Debe extraer lo que funciono, descartar lo que deformo el sistema y dar a cada aprendizaje un lugar mas preciso.

## Que rescata ipv8 de la historia anterior

`ipv8` rescata de `Ipv7IEU`, `Ipv7-6` y `Ipv7-7` la seriedad operacional del gateway, la DHT, la API local, el control plane y la idea de evidencias tecnicas.

`ipv8` rescata de `ipv7c` la intuicion de un futuro mas soberano y mas portable, pero evita repetir la fragmentacion entre lo que promete el README y lo que realmente ejecuta el nodo.

`ipv8` rescata de las ramas de agentes la necesidad de memoria, orquestacion, herramientas y autoauditoria, pero lo mete en un anillo mas seguro y menos invasivo.

## Que rechaza ipv8 de la historia anterior

`ipv8` rechaza el monolito total.
`ipv8` rechaza la duplicidad de nucleos con el mismo nombre y distinto alcance.
`ipv8` rechaza la mezcla desordenada entre manifiesto, runtime, politica y codigo de emergencia.
`ipv8` rechaza la expansion por entusiasmo si no existe un contrato claro entre modulos.

## Resultado arquitectonico esperado

Cuando `ipv8` exista de verdad, cualquier persona o IA deberia poder responder estas preguntas sin perderse.

Que modulo define identidad.
Que modulo mueve datos.
Que modulo decide politicas.
Que modulo descubre peers.
Que modulo explica el estado del sistema.
Que modulo asiste y automatiza.

Si esas respuestas no son obvias, entonces `ipv8` todavia no esta bien construido aunque compile.
