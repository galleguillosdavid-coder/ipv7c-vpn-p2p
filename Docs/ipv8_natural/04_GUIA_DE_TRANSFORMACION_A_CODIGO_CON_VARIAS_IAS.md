# Guia de transformacion a codigo con varias IAs

## Objetivo de esta guia

Esta guia existe para que no intentes construir `ipv8` con una sola IA cargando todo el universo del proyecto al mismo tiempo.
La idea es repartir el sistema en paquetes cognitivos pequenos, pero sin que cada IA invente un proyecto distinto.

## Regla uno: una IA no recibe la vision completa en bruto

La vision completa ya existe en el masterplan y en este paquete natural.
La IA que implemente una pieza no necesita tragarse las cinco mil lineas de una vez.
Necesita recibir solo el contexto correcto de su dominio.

## Regla dos: cada IA trabaja sobre un paquete documental acotado

Si una IA va a trabajar identidad, se le entrega la arquitectura general, el arbol de carpetas relevante y el contrato de identidad.
Si una IA va a trabajar discovery, se le entrega discovery, rutas y observabilidad asociada.
Si una IA va a trabajar desktop, se le entrega la vista de estado, acciones permitidas y contratos de observabilidad.

## Regla tres: antes de pedir codigo, pide reexplicacion

A cada IA primero se le deberia pedir que explique con sus palabras el modulo asignado.
Si la IA no puede reexplicar bien el modulo, todavia no deberia escribir codigo.

## Regla cuatro: despues de reexplicar, pide pseudocodigo humano

No saltes del documento natural al codigo final.
Pide antes una traduccion intermedia en pasos, responsabilidades, estructuras y flujo.
Esa capa intermedia actua como amortiguador de errores.

## Regla cinco: una IA por dominio principal

No conviene repartir la misma carpeta entre demasiadas IAs al mismo tiempo.
Conviene repartir dominios estables.
Una IA para identidad.
Una IA para plano de control.
Una IA para discovery.
Una IA para observabilidad.
Una IA para experiencia desktop.
Otra IA para validacion y revisiones cruzadas.

## Regla seis: existe una IA integradora

Siempre debe haber una IA o una persona que no escriba primero el modulo, sino que compare salidas.
Esa capa integradora verifica nombres, contratos, dependencias y coherencia narrativa.

## Flujo recomendado de trabajo

Primero, elegir un dominio.
Segundo, recopilar solo los documentos naturales relevantes para ese dominio.
Tercero, pedir explicacion resumida del dominio.
Cuarto, pedir propuesta de carpetas y archivos concretos para ese dominio.
Quinto, pedir pseudocodigo o especificacion tecnica intermedia.
Sexto, pedir implementacion parcial.
Septimo, pedir revision contra contratos.
Octavo, integrar.

## Lotes sugeridos para repartir ipv8

### Lote A

Identidad, confianza y continuidad del nodo.

### Lote B

Sesiones, rutas, fragmentacion y transporte.

### Lote C

Grupos, dispositivos, politicas y evaluacion.

### Lote D

Discovery local, discovery remoto, semillas y topologia.

### Lote E

Telemetria, logs, snapshots, auditabilidad y paneles de estado.

### Lote F

Desktop, acciones humanas, experiencia local y explicabilidad.

### Lote G

Runbooks, automatizacion asistida y traduccion tecnico humana.

### Lote H

Migraciones desde `ipv7c` y desde `antiguos`.

## Que debe recibir cada IA como contexto minimo

Nombre del dominio.
Objetivo del dominio.
Que carpetas le pertenecen.
Que carpetas no le pertenecen.
Que contratos debe respetar.
Que nombres no puede cambiar sin propuesta previa.
Que salida se espera en esta fase.

## Que no debe pedirse a una IA de implementacion

No deberia pedirsele que rediseñe toda la arquitectura.
No deberia pedirsele que cambie nombres fundacionales sin consulta.
No deberia pedirsele que implemente de golpe identidad, control plane, UI y laboratorio en una sola entrega.
No deberia pedirsele que mezcle una propuesta de marketing con una estructura tecnica si el objetivo es construir.

## Como detectar que una IA empezo a colapsar

Empieza a repetir nombres sin criterio.
Empieza a inventar carpetas no previstas.
Empieza a mezclar responsabilidades incompatibles.
Empieza a responder con entusiasmo grandioso pero menos precision.
Empieza a olvidar contratos ya fijados.
Empieza a proponer un archivo gigante que lo contiene todo.

## Que hacer cuando una IA colapsa

No corregirla solo con mas texto encima.
Reiniciar el modulo.
Reducir el contexto.
Volver al documento natural exacto del dominio.
Pedir una nueva reexplicacion corta.
Y solo despues reintentar.

## Regla final de soberania humana

Tu eres quien transforma estos documentos en un plan de construccion real entre varias IAs.
Los documentos existen para que tu mantengas el centro del proyecto y no para que una sola salida automatica se apropie del lenguaje de `ipv8`.

## Cierre

Si sigues esta metodologia, `ipv8` deja de ser una bestia imposible para una sola inteligencia y pasa a ser una federacion de problemas manejables, traducibles y verificables.
