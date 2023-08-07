variable "version" {
  type = string
}

job "ip-counter" {
  datacenters = ["sg-ln"]

  group "ip-counter" {
    network {
      port "http" {}
    }

    service {
      name = "ip-counter"
      port = "http"
      tags = [
        "traefik.enable=true",
        "traefik.http.routers.ip-counter.rule=Host(`ipcounter.ilman.io`)",
        "traefik.http.routers.ip-counter.entrypoints=websecure",
        "traefik.http.routers.ip-counter.tls=true",
        "traefik.http.routers.ip-counter.tls.certResolver=cloudflareResolver",
      ]

      check {
        name = "IP Counter HTTP Check"
        type = "http"
        port = "http"
        path = "/health/live"

        interval = "30s"
        timeout  = "10s"
      }
    }

    task "ip-counter" {
      driver = "docker"

      config {
        image = "ilmannafian/ip-counter:${var.version}"
        ports = ["http"]
      }

      env {
        HOST = "0.0.0.0"
        PORT = NOMAD_PORT_http

        RUST_LOG = "info"
      }

      resources {
        cpu    = 10
        memory = 10
      }
    }
  }
}
