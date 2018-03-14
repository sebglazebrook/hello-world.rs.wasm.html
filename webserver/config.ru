use Rack::Static,
  :urls => ["/wasm"],
  :root => "public",
  :header_rules => [
    [:all, { "Content-Type" => "application/wasm" }]
  ]

run lambda { |env|
  [
    200,
    {
      'Content-Type'  => "text/html",
      'Cache-Control' => 'public, max-age=86400'

    },
    File.open('public/index.html', File::RDONLY)

  ]
}
