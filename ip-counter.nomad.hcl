variable "version" {
  type = string
}

job "ip-counter" {
  datacenters = ["id-dpk"]

  group "ip-counter" {
    network {
      mode = "bridge"

      port "healthcheck" {}
    }

    service {
      name = "ip-counter"
      port = "8080"

      connect {
        sidecar_service {}
      }

      check {
        name = "IP Counter HTTP Check"
        type = "http"
        port = "healthcheck"
        path = "/health/live"

        interval = "30s"
        timeout  = "10s"
      }
    }

    task "ip-counter" {
      driver = "docker"

      config {
        image = "ilmannafian/ip-counter:${var.version}"
      }

      env {
        HOST = "127.0.0.1"
        PORT = "8080"

        RUST_LOG = "info"
      }

      resources {
        cpu    = 10
        memory = 10
      }
    }
  }
}
